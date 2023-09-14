// Use crates
extern crate dotenvy;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

mod models;
mod schema;
mod toml_utils;
use toml_utils::{load_toml, toml_schema};

use models::insertable as m_ins;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const STANDARD_TOML_PATH: &str = "mockdb/toml_test.toml";

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
        println!("Running the basic config.");
        populate_db_from_toml(connection, STANDARD_TOML_PATH);
    } else {
        dbg!("Database is not empty! Proceding...");
    }
}

fn is_database_empty(connection: &mut SqliteConnection) -> bool {
    // TODO: get it to work with the actual database, not just check if the file exists.
    use diesel::result::DatabaseErrorKind as dErrorKind;
    use diesel::result::Error as dError;
    use schema::categories::dsl::*;

    match categories.count().first::<i64>(connection) {
        Ok(n) => n == 0, // true if no rows in that table
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

fn populate_db_from_toml(connection: &mut SqliteConnection, toml_path: &str) {
    use schema::categories::dsl::*;
    use schema::options::dsl::*;

    let toml_data = load_toml(toml_path).expect("Couldn't load the data to the TOML.");
    let objects_to_insert = toml_to_db_query(&toml_data);

    // Insert questions.
    diesel::insert_into(categories)
        .values(objects_to_insert.categories)
        .execute(connection)
        .expect("Failed to write to database.");

    // Insert the choices for the questions.
    diesel::insert_into(options)
        .values(objects_to_insert.alternatives)
        .execute(connection)
        .expect("Failed to write to database.");
}

struct ObjectsToInsert {
    categories: Vec<m_ins::Category>,
    alternatives: Vec<m_ins::DBOption>,
    // categories_options: Option<Vec<m_ins::CategoryOption>>,
}

fn toml_to_db_query(toml_data: &toml_schema::TomlData) -> ObjectsToInsert {
    let mut result_questions: Vec<m_ins::Category> = Vec::new();
    let mut result_question_options: Vec<m_ins::DBOption> = Vec::new();

    for question in toml_data.questions.iter() {
        // First, add the question to the db
        let category_type = question.question_type.unwrap_or(1);

        let cat = m_ins::Category {
            label: question.label.clone(),
            prompt: question.prompt.clone(),
            category_type,
            extra_info: question.extra_info.clone(),
        };
        result_questions.push(cat);

        for qo in question.options.iter() {
            let dbo = m_ins::DBOption {
                label: qo.label.clone(),
                shortcut: qo.shortcut.clone(),
            };
            result_question_options.push(dbo);
        }
    }

    ObjectsToInsert {
        categories: result_questions,
        alternatives: result_question_options,
        // categories_options: None,
    }
}
