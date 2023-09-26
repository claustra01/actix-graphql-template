use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
  dotenv().ok();
  let url = env::var("DATABASE_URL").expect("Failed to find DATABASE_URL in .env");

  // create db connection pool
  let manager = ConnectionManager::<PgConnection>::new(url);
  let pool: Pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");
  pool
}