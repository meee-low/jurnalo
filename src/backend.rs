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
        dbg!("Trying to create database from scratch.");
        create_basic_database(connection).unwrap();
        dbg!("Created database.");
    } else {
        dbg!("Database is not empty! Proceding...");
    }
}

fn is_database_empty(connection: &mut SqliteConnection) -> bool {
    // TODO: get it to work with the actual database, not just check if the file exists.
    use diesel::result::DatabaseErrorKind as dErrorKind;
    use diesel::result::Error as dError;
    use schema::entries::dsl::entries;

    match entries.count().first::<i64>(connection) {
        Ok(_) => false, // found *something*, so it's not empty
        Err(dError::DatabaseError(dErrorKind::Unknown, d)) => {
            // This error is typically "table not found."
            // TODO: Keep an watch on this, make sure it's actually always "table not found"
            dbg!("{}", d);
            true
        }
        Err(e) => panic!("{}", e),
    }
}

fn create_basic_database(
    connection: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations.");
    Ok(())
}

// APIs
