mod backend;
mod cli_parsing;
mod errors;
mod models;
mod modes;

use clap::Parser;
use errors::{Error, ParsingCommandError};

// const MOCK_LOG_PATH: &str = "mockdb/logs.txt";

fn main() -> Result<(), Error> {
    backend::setup();

    let args = cli_parsing::Args::parse();

    dbg!(&args);

    cli_parsing::dispatch(&args)?;
    Ok(())
}

// fn parse_and_run_command(command_string: &str, content: &[String]) -> Result<(), Error> {
//     // Keep content as array because it's useful for some commands
//     // that may take multiple commands (e.g.: `jurnalo habit add)
//     use Command as C;
//     let command = Command::from_string(command_string).expect("Command not recognized.");
//     // IDEA: have the Command take in the content and parse it.
//     // Could be useful for multi-word commands e.g.: `jurnalo category add`.
//     match command {
//         C::Full => modes::run_quiz::quiz_full(content),
//         C::QuickNote => modes::quick_note::parse_note(content),
//         C::Habit => {
//             todo!()
//         }
//         C::Print => {
//             // TODO: clean this up
//             let printable = modes::print::printable_entries(
//                 chrono::Utc::now().naive_utc(),
//                 chrono::Utc::now()
//                     .checked_sub_days(chrono::Days::new(7))
//                     .expect("Couldn't subtract days?")
//                     .naive_utc(),
//             )?;
//             println!("{}", printable);
//             Ok(())
//         }
//         C::Export => {
//             todo!()
//         }
//     }
// }

// enum Command {
//     Full,
//     QuickNote,
//     Habit,
//     Print,
//     Export,
// }

// impl Command {
//     fn from_string(command_string: &str) -> Result<Self, ParsingCommandError> {
//         match command_string.to_lowercase().trim() {
//             "full" => Ok(Command::Full),
//             "note" => Ok(Command::QuickNote),
//             "habit" => Ok(Command::Habit),
//             "print" => Ok(Command::Print),
//             "export" => Ok(Command::Export),
//             _ => Err(ParsingCommandError::CommandNotRecognized(
//                 command_string.to_owned(),
//             )),
//         }
//     }
// }
