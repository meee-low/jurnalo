use std::env;

mod backend;
mod errors;
mod models;

use backend::api;
use errors::{Error, ParsingCommandError};

// const MOCK_LOG_PATH: &str = "mockdb/logs.txt";

fn main() -> Result<(), Error> {
    backend::setup();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Insufficient arguments");
            // TODO: Help message. Only returning error for now.
            Err(ParsingCommandError::TooFewArguments.into())
        }
        2.. => {
            // Has command.
            parse_and_run_command(&args[1], &args[2..])
        }
        _ => unreachable!(),
    }
}

fn parse_and_run_command(command_string: &str, content: &[String]) -> Result<(), Error> {
    // Keep content as array because it's useful for some commands
    // that may take multiple commands (e.g.: `jurnalo habit add)
    use Command as C;
    let command = Command::from_string(command_string).expect("Command not recognized.");
    // TODO: Idea: have the Command take in the content and parse it.
    // Could be useful for multi-word commands e.g.: `jurnalo category add`.
    match command {
        C::Full => quiz_full(content),
        C::QuickNote => parse_note(content),
        C::Habit => {
            todo!()
        }
        C::Print => {
            todo!()
        }
        C::Export => {
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
        Err(e) => Err(e),
    }
}

fn add_note(note: String) -> Result<(), Error> {
    if note.is_empty() {
        todo!();
    }
    api::post_entry(None, None, Some(note.clone()))?;
    println!("Success! Saved note:\n{}", note);
    Ok(())
}

fn get_user_input() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => {
            panic!("Couldn't handle user input.")
        }
    }
}

fn quiz_full(content: &[String]) -> Result<(), Error> {
    if !content.is_empty() {
        return Err(ParsingCommandError::TooManyArguments.into());
    }

    let mut inputs = Vec::new();
    let mut entries: Vec<(Option<i32>, Option<i32>, Option<String>)> = Vec::new();

    let categories_and_choices = backend::api::get_categories_and_choices_from_quiz_label("full")?;

    for (cat, choices) in categories_and_choices.iter() {
        println!("{}", cat.prompt);
        match choices {
            Some(choices) => {
                println!(
                    "{}",
                    choices
                        .iter()
                        .map(|c| format!("[{}] {}", c.shortcut, c.label))
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            None => {}
        }
        let input = get_user_input().trim().to_owned();
        inputs.push(input.clone());
        let mut shortcuts: Vec<(i32, String)> = Vec::new();
        for choice in choices.clone().unwrap_or(vec![]) {
            shortcuts.push((choice.id, choice.shortcut.clone()));
        }
        let parsed_input = parse_shortcuts(input, shortcuts);
        for (choice_id, detail) in parsed_input {
            entries.push((Some(cat.id), choice_id, detail));
        }
    }

    println!("{}", inputs.join(" | "));
    backend::api::post_multiple_entries(entries).expect("Faied to add to the database.");
    Ok(())
}

fn parse_shortcuts(
    user_input: String,
    shortcuts: Vec<(i32, String)>,
) -> Vec<(Option<i32>, Option<String>)> {
    // Get all the shortcuts that match the category label.
    // Walk through the string and find the matches.

    let split_input: Vec<&str> = user_input.split(':').collect();

    let mut result: Vec<(Option<i32>, Option<String>)> = Vec::new();

    // TODO: Triple for-loop, could maybe be optimized.
    for (i, input) in split_input.iter().enumerate() {
        if i == 0 {
            // parse it into smaller chunks according to the result
            let maybe_shortcuts: Vec<&str> = input.split(' ').collect();
            let mut found = false;
            for mb in maybe_shortcuts.iter() {
                for (id, s) in shortcuts.iter() {
                    if s.to_lowercase() == mb.to_lowercase().trim() {
                        result.push((Some(*id), None));
                        found = true;
                    }
                }
                if !found {
                    // TODO: Doesn't currently work.
                    result.push((None, Some((*mb).to_owned())));
                }
            }
        } else {
            result.push((None, Some(input.to_string())));
        }
    }

    // TODO: ignore the ones that are (None, None)
    result
}

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
            _ => Err(ParsingCommandError::CommandNotRecognized(
                command_string.to_owned(),
            )),
        }
    }
}
