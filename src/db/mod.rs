use rocket_contrib::databases::diesel;

pub mod posts;
pub mod users;
pub mod comments;

#[database("pg_db")]
pub struct Database(diesel::PgConnection);

pub type Result<T> = std::result::Result<T, diesel::result::Error>;
