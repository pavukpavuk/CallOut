use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderValue, Request, StatusCode},
    Json,
};
// use mail_send::mail_auth::hickory_resolver::proto::rr::rdata::name;
use serde::{Deserialize, Serialize};

use ::sqlx::{prelude::FromRow, Pool, Postgres, Row};

use futures::StreamExt;
use std::sync::Arc;

use tower_cookies::{cookie::SameSite, Cookie, Cookies};

use time::{Duration, OffsetDateTime};

//my imports
use crate::{
    crypto::Crypto,
    logging::{self},
    shb_error::BackendError,
    state::AppState,
    user_accounts::{
        session::{extract_session_id_from_cookie, get_session_from_id},
        user_validation,
    },
};

use rand::{rngs::OsRng, RngCore};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

// use mail_send::{
//     SmtpClientBuilder
// };
// use mail_builder::{
//     MessageBuilder
// };

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    id: i32,
    username: String,
    email: Vec<u8>,
    email_nonce: Vec<u8>,
    password_hash: String,
    user_key: Vec<u8>,
    user_key_nonce: Vec<u8>,
    verified: bool,
}

// struct User{
//     id: i32,
//     username:String,
//     email: EncryptedData,
//     user_key: EncryptedData,
//     password_hash:String,
//     verified: bool
// }

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct UserLoginDetails {
    pub username: String,
    pub email: Option<String>,
    #[sqlx(rename = "pword")]
    pub password: String,
}

#[derive(Serialize)]
pub struct UserPrivate {
    pub username: String,
    pub email: String,
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserPublic {
    username: String,
}

/*
    todo::
    email:
        1: verification #YOXJAZ
            2a: html template email
            2b: verification code/link

    verification tokens #YPHXZH
*/
pub async fn create_user(
    app_state: State<Arc<AppState>>,
    Json(new_user): Json<UserLoginDetails>,
) -> Result<StatusCode, BackendError> {
    let new_user_email = new_user.email.clone();
    let new_user_email = new_user_email
        .ok_or_else(|| BackendError::UnprocessableEntity("Unprocessable entity".to_string()))?;

    if new_user.username.is_empty() || new_user_email.is_empty() || new_user.password.is_empty() {
        return Err(BackendError::UnprocessableEntity(
            "All fields need to be submitted".to_string(),
        ));
    }

    user_validation::validate_email(&new_user_email)?;
    user_validation::validate_password(&new_user)?;

    //user email validation
    {

        // let recipient_email = env::var("RECIPIENT_EMAIL").expect("recipient_email not set.");
        // let sender_email = env::var("SENDER_EMAIL").expect("sender_email not set.");
        // let smtp_password = env::var("SMTP_PASSWORD").expect("smtp_password not set.");

        // let user_message = "<h1 style='color:pink'>Smello beka. Just created user ".to_string() + &new_user.username + "</h1>";
        // let message = MessageBuilder::new()
        //     .from(("Mr Big Business", sender_email.as_str()))
        //     .to(vec![
        //         ("Beka", recipient_email.as_str() ),
        //     ])
        //     .subject("Created a new user!")
        //     .html_body(user_message.clone())
        //     .text_body(user_message);

        // // Connect to the SMTP submissions port, upgrade to TLS and
        // // authenticate using the provided credentials.
        // SmtpClientBuilder::new("smtp.gmail.com", 587)
        //     .implicit_tls(false)
        //     .credentials((sender_email.as_str(), smtp_password.as_str()))
        //     .connect()
        //     .await
        //     .unwrap()
        //     .send(message)
        //     .await
        //     .unwrap();
    }

    //hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash_to_store =
        Argon2::hash_password(&app_state.argon_ctx, new_user.password.as_bytes(), &salt).map_err(
            |e| {
                logging::log_users(format!("Failed to hash password for user: {}", e).as_str());
                BackendError::InternalServerError("Internal server error".to_string())
            },
        )?;

    //encrypt email
    let crypto_with_user_key = Crypto::new_with_random_key();

    let email_encrypted = crypto_with_user_key
        .encrypt(&new_user_email.as_bytes())
        .map_err(|e| {
            logging::log_users(format!("Failed to encrypt email for user: {}", e).as_str());
            BackendError::InternalServerError("Internal server error".to_string())
        })?;

    let crypto_with_master_key =
        Crypto::new_with_provided_key_hex(&app_state.master_key.as_bytes()).map_err(|e| {
            logging::log_users(format!("Failed to instantiate Crypto instance: {}", e).as_str());
            BackendError::InternalServerError("Internal server error".to_string())
        })?;

    //encrypt user key
    let user_key_encrypted = crypto_with_master_key
        .encrypt(&crypto_with_user_key.key())
        .map_err(|e| {
            logging::log_users(format!("Failed to encrypt user key: {}", e).as_str());
            BackendError::InternalServerError("Internal server error".to_string())
        })?;

    let query = sqlx::query_file_as!(
        UserRow,
        "src/user_accounts/sql/insert_user.sql",
        new_user.username.as_str(),
        email_encrypted.ciphertext(),
        password_hash_to_store.to_string(),
        email_encrypted.nonce(),
        user_key_encrypted.ciphertext(),
        user_key_encrypted.nonce()
    )
    .execute(&app_state.db_pool)
    .await;

    match query {
        Ok(_res) => {
            logging::log_users(format!("User {} inserted", new_user.username.clone()).as_str());
            Ok(StatusCode::OK)
        }
        Err(err) => match err.as_database_error() {
            Some(err) => {
                println!("DB error: {}", err);
                if err.code() == Some("23505".into()) {
                    return Err(BackendError::Conflict(
                        "Username or email already exists".to_string(),
                    ));
                }
                return Err(BackendError::InternalServerError(
                    "Unknown error".to_string(),
                ));
            }
            None => {
                println!("DB error: Unknown");
                return Err(BackendError::InternalServerError(
                    "Unknown error".to_string(),
                ));
            }
        },
    }
}

pub async fn find_all_users(
    app_state: State<Arc<AppState>>,
) -> Result<Json<Vec<UserPublic>>, BackendError> {
    let mut all_users: Vec<UserPublic> = Vec::new();

    let mut query_result =
        sqlx::query("SELECT username FROM useraccounts;").fetch(&app_state.db_pool);

    while let Some(row_result) = query_result.next().await {
        match row_result {
            Ok(row) => {
                all_users.push(UserPublic {
                    username: row.get("username"),
                });
            }
            Err(err) => match err.as_database_error() {
                Some(err) => {
                    logging::log_users(format!("Error: {}", err).as_str());
                    return Err(BackendError::InternalServerError(
                        "Internal server error".to_string(),
                    ));
                }
                None => {
                    logging::log_users(format!("Error: Unknown database error").as_str());
                    return Err(BackendError::InternalServerError(
                        "Internal server error".to_string(),
                    ));
                }
            },
        }
    }

    Ok(Json(all_users))
}

/*
 Todo::
    1: Implement correct error handling.
    2: Create session creation abstraction
*/
pub async fn user_login(
    app_state: State<Arc<AppState>>,
    cookies: Cookies,
    Json(user_login_details): Json<UserLoginDetails>,
) -> Result<StatusCode, BackendError> {
    //verify user START
    if user_login_details.username.is_empty() || user_login_details.password.is_empty() {
        return Err(BackendError::UnprocessableEntity(
            "All fields must contain a value".to_string(),
        ));
    }

    let user = get_user_from_username(&user_login_details.username, &app_state.db_pool)
        .await
        .map_err(|_e| BackendError::NotFound("User not found".to_string()))?;

    let found_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| BackendError::InternalServerError("Internal Server error".to_string()))?;

    Argon2::verify_password(
        &app_state.argon_ctx,
        user_login_details.password.as_bytes(),
        &found_hash,
    )
    .map_err(|_| BackendError::NotAuthorized("Password does not match".to_string()))?;
    //verify user END

    //now successfully verified

    //create session START
    let now = OffsetDateTime::now_utc();
    let expires = now + Duration::minutes(10);

    let mut session_id = [0u8; 32];
    OsRng.try_fill_bytes(&mut session_id).map_err(|e| {
        logging::log_users(format!("Could not generate session id: {}", e).as_str());
        BackendError::InternalServerError("Could not generate session id".to_string())
    })?;

    //add session to session table in DB
    let query = sqlx::query_file!(
        "src/user_accounts/sql/insert_session.sql",
        &session_id,
        user.id,
        expires
    )
    .execute(&app_state.db_pool)
    .await;

    match query {
        Ok(_res) => {
            logging::log(&format!("Session created for user: {}.", user.username));
        }
        Err(e) => {
            logging::log_users(&format!("Session creation failed: {}", e));
            return Err(BackendError::InternalServerError(
                "Session creation failed".to_string(),
            ));
        }
    }
    //create session END -> return session id

    let session_cookie = Cookie::build(("session", hex::encode(session_id)))
        .path("/")
        .http_only(true)
        .same_site(SameSite::None)
        .secure(true)
        .expires(expires)
        .build();

    cookies.add(session_cookie);

    Ok(StatusCode::OK) //user is logged in
}

fn decrypt_user_data(
    app_state: State<Arc<AppState>>,
    user: &mut User,
) -> Result<&mut User, BackendError> {
    let generic_error = BackendError::InternalServerError("Internal Server Error".to_string());

    //use master key to decrypt user key
    let crypto_master_key = Crypto::new_with_provided_key_hex(&app_state.master_key.as_bytes())
        .map_err(|_| {
            println!("failed to create master key");
            generic_error.clone()
        })?;

    let decrypted_user_key = crypto_master_key
        .decrypt(&user.user_key, &user.user_key_nonce)
        .map_err(|_| {
            println!("failed to decrypt user key");
            generic_error.clone()
        })?;

    let crypto_user_key =
        Crypto::new_with_provided_key(&decrypted_user_key.as_bytes()).map_err(|_| {
            println!("failed to instantiate user key crypto");
            generic_error.clone()
        })?;

    //more to come i assume
    user.email = crypto_user_key
        .decrypt(&user.email, &user.email_nonce)
        .map_err(|_| {
            println!("failed to decrypt user email");
            generic_error.clone()
        })?
        .as_bytes();

    Ok(user)
}

/*
    todo::
        1: Could do with some cleaning
        2: Proper error handling
        3: Abstract into middleware
        4: ABSTRACT PLEASE
*/

async fn get_user_from_session_cookie(
    db_pool: &Pool<Postgres>,
    cookies: Cookies,
) -> Result<User, BackendError> {
    let session_id = extract_session_id_from_cookie(cookies)?;
    let user_session = get_session_from_id(&db_pool, &session_id).await?;

    get_user_from_id(&user_session.user_id, &db_pool)
        .await
        .map_err(|e| BackendError::InternalServerError(format!("Could not retrieve user: {}", e)))
}

pub async fn get_private_user(
    app_state: State<Arc<AppState>>,
    cookies: Cookies,
    req: Request<Body>,
) -> Result<Json<UserPrivate>, BackendError> {
    let mut user = get_user_from_session_cookie(&app_state.db_pool, cookies).await?;

    let user = decrypt_user_data(app_state, &mut user)?;

    let username = user.username.clone();
    let user_email = String::from_utf8(user.email.clone())
        .map_err(|e| BackendError::InternalServerError(format!("{}", e.to_string())))?;

    let (parts, _) = req.into_parts();

    let user_agent = parts
        .headers
        .get("user-agent")
        .unwrap_or(&HeaderValue::from_static("user agent not found"))
        .to_str()
        .map_err(|e| {
            BackendError::InternalServerError(format!(
                "Failed to parse user agent: {}",
                e.to_string()
            ))
        })?
        .to_string();

    let user = UserPrivate {
        email: user_email,
        username: username,
        user_agent: String::from(user_agent),
    };

    Ok(Json(user))
}

pub async fn user_logout(
    app_state: State<Arc<AppState>>,
    cookies: Cookies,
) -> Result<StatusCode, BackendError> {
    let session_cookie = cookies
        .get("session")
        .ok_or_else(|| BackendError::NotAuthorized("Session has expired".to_string()))?;

    let session_id_hex = session_cookie.value();
    let session_id_bytes = hex::decode(session_id_hex)
        .map_err(|_| BackendError::InternalServerError("Error logging out".to_string()))?;

    cookies.remove(Cookie::build("session").path("/").build());

    sqlx::query_file!("src/user_accounts/sql/revoke_session.sql", session_id_bytes)
        .execute(&app_state.db_pool)
        .await
        .map_err(|_| BackendError::InternalServerError("Unable to logout".to_string()))?;

    Ok(StatusCode::OK)
}

async fn get_user_from_username(
    username_to_search_for: &str,
    db_pool: &Pool<Postgres>,
) -> Result<User, String> {
    let query = sqlx::query_as!(
        User,
        "SELECT * FROM useraccounts WHERE username=$1 LIMIT 1;",
        username_to_search_for
    )
    .fetch_one(db_pool)
    .await;

    match query {
        Ok(found_user_row) => {
            //todo:: return actual user struct instead of user row struct
            return Ok(found_user_row);
        }
        Err(err) => match err.as_database_error() {
            Some(err) => {
                logging::log(format!("Error finding user: {}", err).as_str());
                return Err(format!("Error finding user: {}", err));
            }
            None => {
                return Err("User does not exist".to_string());
            }
        },
    }
}

async fn get_user_from_id(user_id: &i32, db_pool: &Pool<Postgres>) -> Result<User, String> {
    let query = sqlx::query_as!(
        User,
        "SELECT * FROM useraccounts WHERE id=$1 LIMIT 1;",
        user_id
    )
    .fetch_one(db_pool)
    .await;

    match query {
        Ok(found_user_row) => {
            //todo:: return actual user struct instead of user row struct
            return Ok(found_user_row);
        }
        Err(err) => match err.as_database_error() {
            Some(err) => {
                logging::log(format!("Error finding user: {}", err).as_str());
                return Err(format!("Error finding user: {}", err));
            }
            None => {
                return Err("User does not exist".to_string());
            }
        },
    }
}

/*
    todo::
        1: regex for finding users similar to the one submitted
*/
pub async fn find_user(
    app_state: State<Arc<AppState>>,
    path: Path<String>,
) -> Result<Json<Vec<UserPublic>>, BackendError> {
    let username_to_search_for = path.0;
    // println!("looking for user: {}", username_to_search_for);

    let mut query_found_users = sqlx::query_file!(
        "src/user_accounts/sql/find_users.sql",
        username_to_search_for
    )
    .fetch(&app_state.db_pool);
    let mut found_users: Vec<UserPublic> = Vec::new();

    while let Some(row_result) = query_found_users.next().await {
        match row_result {
            Ok(row) => {
                found_users.push(UserPublic {
                    username: row.username,
                });
            }
            Err(err) => match err.as_database_error() {
                Some(err) => {
                    println!("Error: {}", err);
                    return Err(BackendError::InternalServerError(
                        "Internal server error".to_string(),
                    ));
                }
                None => {
                    println!("Error: Unknown database error");
                    return Err(BackendError::InternalServerError(
                        "Internal server error".to_string(),
                    ));
                }
            },
        }
    }

    if found_users.is_empty() {
        return Err(BackendError::NotFound("No users found".to_string()));
    }

    Ok(Json(found_users))
}

/*
   todo::
       1: Implement some sort of "admin password" to control user accounts
           1a: use the admin account/password to gatekeep deletion of user accounts.
           ?a: use the admin account/password to gatekeep CREATION of user accounts.
*/
pub async fn _remove_user() -> StatusCode {
    //remove from database
    StatusCode::IM_A_TEAPOT
}
