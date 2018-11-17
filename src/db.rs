use rocket_contrib::databases::diesel;

#[database("pg_db")]
pub struct Database(diesel::PgConnection);
