use sqlx::postgres::PgPoolOptions;

pub type DBConnection = sqlx::Pool<sqlx::Postgres>;
pub type DbErr = sqlx::Error;

pub struct Connection;

impl Connection {
    pub fn lazy() -> Result<DBConnection, DbErr> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in env.");
        PgPoolOptions::new().connect_lazy(&database_url)
    }

    pub async fn connect() -> Result<DBConnection, DbErr> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in env.");
        PgPoolOptions::new().connect(&database_url).await
    }
}
