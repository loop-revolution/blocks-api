use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use log::error;
use r2d2::{Pool, PooledConnection};
use std::env;

/// Type for the access to the DB
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type PgConnect = PooledConnection<ConnectionManager<PgConnection>>;

/// Makes a connection to the DB
pub fn get_pool(url: &str) -> PostgresPool {
	// These lines create a connection pool
	let mng = ConnectionManager::<PgConnection>::new(url);
	r2d2::Pool::builder()
		.build(mng)
		.expect("could not build connection pool")
}

/// Gets the DATABASE_URL from the environment
pub fn env_db() -> String {
	// Load the environment from .env file
	dotenv().ok();
	// Postgres DB URL (from env as `DATABASE_URL`)
	match env::var("DATABASE_URL") {
		Ok(url) => url,
		Err(_) => {
			error!("A 'DATABASE_URL' environment variable is required.");
			panic!()
		}
	}
}

/// Gets the DATABASE_URL from the environment, if
/// the INTROSPECTING environment variable is not "true"
pub fn optional_env_db() -> Option<String> {
	// Load the environment from .env file
	dotenv().ok();
	// If introspection is all that is needed,
	// connection to a database is optional.
	match env::var("INTROSPECTING") {
		Ok(val) if val == "true" => return None,
		_ => (),
	}
	// Postgres DB URL (from env as `DATABASE_URL`)
	match env::var("DATABASE_URL") {
		Ok(url) => Some(url),
		Err(_) => {
			error!("A 'DATABASE_URL' environment variable is required.");
			panic!()
		}
	}
}
