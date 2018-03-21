use std::env;
use dotenv::dotenv;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use r2d2;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

// An alias to the type for a pool of Diesel SQLite connections.
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool.
pub fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url.as_str());
    r2d2::Pool::new(manager)
        .expect(&format!("Error connecting pool to {}", database_url))
}
// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Database(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Database {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Database, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Database(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Database as an &SqliteConnection.
impl Deref for Database {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
