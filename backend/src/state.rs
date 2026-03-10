use argon2::Argon2;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub db_url: String,
    pub db_pool: Pool<Postgres>,
    pub argon_ctx: Argon2<'static>,
    pub master_key: String,
}
