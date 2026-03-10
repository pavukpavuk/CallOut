extern crate dotenv;
use argon2::Argon2;
use axum::http::{HeaderValue, Method};
use dotenv::dotenv;
use std::env;

use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use axum::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware,
    routing::get,
    routing::post,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;

use std::net::SocketAddr;

use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod admin;
mod crypto;
mod fighters;
mod logging;
mod shb_error;
mod state;
mod user_accounts;

use state::AppState;

#[tokio::main]

/*todo::
    Correct error handling for main function

*/
async fn main() {
    dotenv().ok();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let cert_pem_url = env::var("CERT_PEM_URL").expect("CERT_PEM_URL not set.");
    let key_pem_url = env::var("KEY_PEM_URL").expect("KEY_PEM_URL not set.");

    let _tls_config = RustlsConfig::from_pem_file(cert_pem_url, key_pem_url)
        .await
        .unwrap();

    let allowed_origins = [
        //machines allowed to access the resources and make requests
        HeaderValue::from_static("http://localhost:5173"),
    ];

    let cors_layer = CorsLayer::new()
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    let db_url = env::var("DATABASE_URL").expect("DB URL not set.");
    let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let master_key = env::var("MASTER_KEY").expect("master_key not set.");

    let app_state = Arc::new(AppState {
        db_url: db_url.clone(),
        db_pool: pool,
        argon_ctx: Argon2::default(),
        master_key: master_key,
    });

    let admin = Router::new()
        .route("/", get(admin::hello_admin))
        .layer(middleware::from_fn(admin::auth_admin));

    let fighters_router = Router::new().route("/", get(fighters::get_fighters));

    let users_router = Router::new()
        .route(
            "/",
            post(user_accounts::create_user).get(user_accounts::find_all_users),
        )
        .route("/{id}", get(user_accounts::find_user))
        .route(
            "/session",
            get(user_accounts::get_private_user)
                .post(user_accounts::user_login)
                .delete(user_accounts::user_logout),
        );

    let app = Router::new()
        .nest("/api/admin", admin)
        .nest("/api/users", users_router)
        .nest("/api/fighters", fighters_router)
        .with_state(app_state)
        .layer(cors_layer)
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3629));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // axum_server::bind_rustls(addr, tls_config)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
