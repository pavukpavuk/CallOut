use time::{
    OffsetDateTime,
    UtcDateTime
};

use::sqlx::{
    Pool,
    Postgres,
};

use crate::{
    logging::{self}, 
    shb_error::BackendError, user_accounts::users::User, 
};
use tower_cookies::{
    Cookies, 
};


#[allow(dead_code)]
pub struct UserSession{
    pub user: User,
    pub session: Session
}
#[allow(dead_code)]
pub struct Session{
    pub id: i32,
    pub session_id: Vec<u8>,
    pub user_id: i32,
    pub created_at: Option<OffsetDateTime>,
    pub expires_at: UtcDateTime,
    pub last_seen_at : Option<OffsetDateTime>,
}

//improve db error handling (make handle_db_error function?)
pub async fn get_session_from_id(db_pool: &Pool<Postgres> ,session_id: &[u8]) -> Result< Session , BackendError>{
    let query = sqlx::query_file_as!( Session, "src/user_accounts/sql/retrieve_session.sql",session_id);
    query.fetch_one(db_pool) 
        .await
        .map_err(|e| {
        logging::log_users(&format!("Session not found: {}", e));
        BackendError::NotFound("Session not found".to_string())
    })
}

pub fn extract_session_id_from_cookie(cookies: Cookies) -> Result<Vec<u8>, BackendError>{ 
    let session_cookie = cookies.get("session").ok_or_else(| | {
        BackendError::NotAuthorized("Session has expired".to_string())
    })?;
    
    let session_id_hex = session_cookie.value();
    hex::decode(session_id_hex).map_err(|_| {
        BackendError::NotAuthorized("Session id is corrupted".to_string())
    })
}