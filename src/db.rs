use anyhow::Context;
use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;
use std::env;

pub fn connection() -> anyhow::Result<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    PgConnection::establish(&database_url)
        .context(format!("Failed to connect {}", database_url))
}