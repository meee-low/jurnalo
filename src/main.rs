use std::env;
use std::fs::File;
use std::io::Write;

use json::JsonValue;

mod errors;
mod json_utils;

use errors::{Error, ParsingCommandError};
use json_utils::*;

const MOCK_LOG_PATH: &str = "mockdb/logs.txt";
const MOCK_SETTINGS_PATH: &str = "mockdb/settings.json";

fn main() -> Result<(), errors::Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => {
            unreachable!()
        }
        1 => {
            println!("Insufficient arguments");
            // TODO: Help message. Only returning error for now.
            return Err(ParsingCommandError::TooFewArguments.into());
        }
        _ => {
            // Has command.
            parse_and_run_command(&args[1], &args[2..])?;
        }
    }
    Ok(())
}

fn parse_and_run_command(command_string: &str, content: &[String]) -> Result<(), Error> {
    // Keep content as array because it's useful for some commands
    // that may take multiple commands (e.g.: `jurnalo habit add)
    use Command::*;
    let command = Command::from_string(command_string).expect("Command not recognized.");
    // TODO: Idea: have the Command take in the content and parse it.
    // Could be useful for multi-word commands e.g.: `jurnalo category add`.
    match command {
        Full => full_battery(content),
        QuickNote => parse_note(content),
        Habit => {
            todo!()
        }
        Print => {
            todo!()
        }
        Export => {
            todo!()
        }
    }
}

fn parse_note(content: &[String]) -> Result<(), Error> {
    let message = content.join(" ");
    if message.is_empty() {
        println!("You must provide a message with this command.")
    }
    match add_note(message) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn add_note(note: String) -> std::io::Result<()> {
    // Currently just a dummy implementation for testing.
    // TODO: Needs to be connected to database.
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(MOCK_LOG_PATH)?;
    writeln!(&mut file, "{}", note)?;
    Ok(())
}

fn full_battery(content: &[String]) -> Result<(), Error> {
    // TODO: Idea: Maybe allow for variants of this? Probe the database for the keyword.
    // E.g.: jurnalo full day, jurnalo full week, jurnalo full social, ...
    // Then it's probably a good idea to just have the command itself (jurnalo day, jurnalo week, ...)
    // For now, complaining when there are extra words.
    if !content.is_empty() {
        return Err(ParsingCommandError::TooManyArguments.into());
    }

    let settings_raw =
        std::fs::read_to_string(MOCK_SETTINGS_PATH).expect("Couldn't open the settings JSON.");
    let settings = json::parse(&settings_raw).expect("Couldn't parse the JSON for questions.");

    let questions: &Vec<JsonValue> =
        get_key_from_object_as_vec(&settings, "questions")?.expect("Key not found");
    let mut inputs: Vec<String> = Vec::new();

    for question in questions.iter() {
        let prompt: &str = get_key_from_object_as_str(question, "prompt")?.expect("Key not found");
        let options: &Vec<JsonValue> =
            get_key_from_object_as_vec(question, "options")?.expect("Key not found");

        let mut shortcut_label_pairs: Vec<(&str, &str)> = Vec::new();
        for option in options.iter() {
            let shortcut = get_key_from_object_as_str(option, "shortcut")?.expect("Key not found");
            let label = get_key_from_object_as_str(option, "label")?.expect("Key not found");
            shortcut_label_pairs.push((shortcut, label));
        }

        println!("{}", prompt);
        println!(
            "{}",
            shortcut_label_pairs
                .iter()
                .map(|(s, l)| format!("[{}] {}", s, l))
                .collect::<Vec<String>>()
                .join(" ")
        );

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
            }
            Err(_) => {
                panic!("Couldn't handle user input.")
            }
        }
        inputs.push(input.trim().to_owned());
    }
    println!("Your answers were: {}", inputs.join(" | "));
    // post_full_entry(serialize_entry(inputs))
    Ok(())
}

// fn serialize_entry(entry: ()) {
//     todo!()
// }

// fn post_full_entry(entry: ()) {
//     todo!()
// }

enum Command {
    Full,
    QuickNote,
    Habit,
    Print,
    Export,
}

impl Command {
    fn from_string(command_string: &str) -> Result<Self, ParsingCommandError> {
        match command_string.to_lowercase().trim() {
            "full" => Ok(Command::Full),
            "note" => Ok(Command::QuickNote),
            "habit" => Ok(Command::Habit),
            "print" => Ok(Command::Print),
            "export" => Ok(Command::Export),
            _ => Err(ParsingCommandError::CommandNotRecognized),
        }
    }
}
