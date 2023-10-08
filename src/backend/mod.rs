// Use crates
extern crate dotenvy;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

use crate::models;
pub mod api;
pub mod schema;
// mod toml_utils;
mod initial_setup;
use initial_setup::toml_utils::{load_toml, toml_schema};

use models::insertable as m_ins;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
// const STANDARD_TOML_PATH: &str = "mockdb/toml_test.toml";
const PRAGMAS: [&str; 1] = ["PRAGMA foreign_keys = ON"];

pub fn setup() {
    // Setup
    let mut connection = establish_connection();

    // If database doesn't exist, create it based on the schema and populate the database with the starting values from the TOML
    create_database_if_it_doesnt_exist(&mut connection);
}

/// Establishes a connection to the database, and returns the connection.
pub fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();
    let database_path: String = env::var("DATABASE_URL").expect("`DATABASE_URL` not set in .env");
    let mut connection = SqliteConnection::establish(&database_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_path));
    for pragma in PRAGMAS {
        // connection.execute(pragma);
        diesel::sql_query(pragma)
            .execute(&mut connection)
            .unwrap_or_else(|_| panic!("Could not execute pragma `{}`", pragma));
    }
    connection
}

pub fn create_database_if_it_doesnt_exist(connection: &mut SqliteConnection) {
    if !is_database_empty(connection) {
        println!("Database is not empty! Proceding...");
        return;
    }
    println!("Trying to create database from scratch.");
    create_basic_database(connection).unwrap();
    println!("Created database.");

    println!("Running the basic config.");
    dotenvy::dotenv().ok();
    let toml_path = env::var("TEST_TOML").expect("`TEST_TOML` not set in .env");
    populate_db_from_toml(connection, &toml_path);
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
    use schema::choices::dsl::*;
    use schema::quizzes::dsl::*;
    use schema::quizzes_to_categories::dsl::*;

    let toml_data = match load_toml(toml_path) {
        Ok(td) => td,
        Err(_) => {
            eprintln!("Couldn't load the data to the TOML.");
            dbg!(toml_path);
            panic!()
        }
    };
    let objects_to_insert = toml_to_db_query(&toml_data);

    diesel::insert_into(categories)
        .values(objects_to_insert.categories)
        .execute(connection)
        .expect("Failed to write to database.");

    diesel::insert_into(choices)
        .values(objects_to_insert.alternatives)
        .execute(connection)
        .expect("Failed to write to database.");

    diesel::insert_into(quizzes)
        .values(objects_to_insert.quizzes)
        .execute(connection)
        .expect("Failed to write quizzes to database.");

    diesel::insert_into(quizzes_to_categories)
        .values(objects_to_insert.quiz_to_cat)
        .execute(connection)
        .expect("Failed to write quiz_to_cat to database.");
    // TODO: Handle foreign key errors, as these are user errors. ErrorType: `DatabaseError(ForeignKeyViolation, _)`
}

struct ObjectsToInsertFromSetup {
    categories: Vec<m_ins::NewCategory>,
    alternatives: Vec<m_ins::NewChoice>,
    quizzes: Vec<m_ins::NewQuiz>,
    quiz_to_cat: Vec<m_ins::NewQuizToCategory>,
}

#[allow(clippy::needless_late_init)] // reason: keep the code more readable by having the similar variables declared in the same place
fn toml_to_db_query(toml_data: &toml_schema::TomlData) -> ObjectsToInsertFromSetup {
    let mut result_questions: Vec<m_ins::NewCategory> = Vec::new();
    let mut result_question_options: Vec<m_ins::NewChoice> = Vec::new();
    let result_quizzes: Vec<m_ins::NewQuiz>;
    let mut result_quiz_to_cat: Vec<m_ins::NewQuizToCategory> = Vec::new();

    for question in toml_data.categories.iter() {
        // First, add the question to the db
        let category_type = question.question_type.unwrap_or(1);

        let cat = m_ins::NewCategory {
            label: question.label.clone(),
            prompt: question.prompt.clone(),
            category_type,
            extra_info: question.extra_info.clone(),
            show_in_streaks: question.show_in_streaks,
            reminder_timer_in_days: question.reminder_timer_in_days,
        };
        result_questions.push(cat);

        match &question.choices {
            None => {}
            Some(choices) => {
                for choice in choices.iter() {
                    result_question_options.push(m_ins::NewChoice {
                        label: choice.label.clone(),
                        shortcut: choice.shortcut.clone(),
                        category_label: question.label.clone(),
                        show_in_streaks: choice.show_in_streaks,
                        reminder_timer_in_days: choice.reminder_timer_in_days,
                    })
                }
            }
        }

        // for qo in question.choices.iter() {
        //     let dbo = m_ins::NewChoice {
        //         label: qo.label.clone(),
        //         shortcut: qo.shortcut.clone(),
        //         category_label: question.label.clone(),
        //     };
        //     result_question_options.push(dbo);
        // }
    }

    result_quizzes = toml_data
        .quizzes
        .iter()
        .map(|q| m_ins::NewQuiz {
            label: q.command.clone(),
            command: Some(q.command.clone()),
        })
        .collect();

    for quiz in toml_data.quizzes.iter() {
        let quiz_label = quiz.command.clone();
        for (i, cat) in quiz.categories.iter().enumerate() {
            result_quiz_to_cat.push(m_ins::NewQuizToCategory {
                quiz_label: quiz_label.clone(),
                category_label: cat.clone(),
                order: i
                    .try_into()
                    .expect("Couldn't convert from usize to i32 in the toml setup."),
            })
        }
    }

    ObjectsToInsertFromSetup {
        categories: result_questions,
        alternatives: result_question_options,
        quizzes: result_quizzes,
        quiz_to_cat: result_quiz_to_cat,
    }
}
