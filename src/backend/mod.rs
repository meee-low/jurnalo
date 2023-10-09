// Use crates
extern crate dotenvy;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub mod api;
pub mod schema;
// mod toml_utils;
mod initial_setup;

// pub use initial_setup::setup;

// const STANDARD_TOML_PATH: &str = "mockdb/toml_test.toml";
const PRAGMAS: [&str; 1] = ["PRAGMA foreign_keys = ON"];

/// Establishes a connection to the database, and returns the connection.
/// If the database_path is None, it will try to get the path from the settings or from the .env file.
pub fn establish_connection(database_path: Option<&str>) -> SqliteConnection {
    let database_path: String = match database_path {
        Some(path) => path.to_string(),
        None => get_database_path(),
    };

    let mut connection = SqliteConnection::establish(&database_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_path));

    for pragma in PRAGMAS {
        diesel::sql_query(pragma)
            .execute(&mut connection)
            .unwrap_or_else(|_| panic!("Could not execute pragma `{}`", pragma));
    }
    connection
}

fn get_database_path() -> String {
    // TODO: Search in config file first.

    // Search in .env
    dotenvy::dotenv().ok();
    env::var("DATABASE_URL").expect("`DATABASE_URL` not set in .env")
}

pub fn setup(initial_settings_path: &Option<String>, database_path: &Option<String>) {
    // TODO: check if database already exists

    // Now it doesn't exist, so we need to create it.
    if let (Some(settings_path), Some(database_path)) = (initial_settings_path, database_path) {
        initial_setup::setup(settings_path, database_path);
        return;
    }

    todo!("We don't support creating the database without a settings file yet. Please pass both the settings file and the database path.");
}
