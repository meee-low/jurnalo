// Use crates
extern crate dotenvy;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

mod models;
mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn main() {
    // Setup
    dotenvy::dotenv().ok();
    let database_path: String = env::var("DATABASE_URL").expect("`DATABASE_URL` not set in .env");
    let mut connection = SqliteConnection::establish(&database_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_path));

    // If database doesn't exist, create it based on the schema and populate the database with the starting values from the TOML
    create_database_if_it_doesnt_exist(&mut connection);
}

pub fn create_database_if_it_doesnt_exist(connection: &mut SqliteConnection) {
    if is_database_empty(connection) {
        create_basic_database(connection).unwrap()
    }
}

fn is_database_empty(_connection: &mut SqliteConnection) -> bool {
    // TODO: get it to work with the actual database, not just check if the file exists.
    // use schema::entries::dsl::entries;
    // use diesel::result::Error as dError;
    // use diesel::result::DatabaseErrorInformation as dErrorInfo;

    // let count: i64 = match entries.count().first(connection){
    //     Ok(v) => v,
    //     Err(dError::DatabaseError(Unknown, d)) => return true,
    //     Err(e) => panic!(e);
    // };
    // count == 0

    let database_path: String = env::var("DATABASE_URL").expect("`DATABASE_URL` not set in .env");
    let path = std::path::Path::new(&database_path);
    !path.exists()
}

fn create_basic_database(
    connection: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations.");
    Ok(())
}

// Tables/Schema

// Structs

// APIs
