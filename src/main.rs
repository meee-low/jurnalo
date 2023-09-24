use itertools::Itertools;
use std::env;

mod backend;
mod errors;
mod models;

use backend::api::{self, EntryWithLabelsTuple};
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
            // TODO: clean this up
            let printable = printable_entries(
                chrono::Utc::now().naive_utc(),
                chrono::Utc::now()
                    .checked_sub_days(chrono::Days::new(7))
                    .expect("Couldn't subtract days?")
                    .naive_utc(),
            )?;
            println!("{}", printable);
            Ok(())
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
        if let Some(cs) = choices {
            println!(
                "{}",
                cs.iter()
                    .map(|c| format!("[{}] {}", c.shortcut, c.label))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }

        let input = get_user_input().trim().to_owned();
        inputs.push(input.clone());

        let shortcuts: Vec<(i32, String)> = choices
            .clone()
            .unwrap_or(vec![])
            .iter()
            .map(|choice| (choice.id, choice.shortcut.clone()))
            .collect();

        let (parsed_choices, parsed_detail) = extract_shortcuts_from_input(input, shortcuts);
        if let Some(cs) = parsed_choices {
            for choice_id in cs {
                entries.push((Some(cat.id), Some(choice_id), parsed_detail.clone()));
            }
        } else if parsed_detail.is_some() {
            entries.push((Some(cat.id), None, parsed_detail));
        }
    }

    println!("{}", inputs.join(" | "));
    backend::api::post_multiple_entries(entries).expect("Failed to add to the database.");
    Ok(())
}

fn extract_shortcuts_from_input(
    user_input: String,
    shortcuts: Vec<(i32, String)>,
) -> (Option<Vec<i32>>, Option<String>) {
    if user_input.is_empty() {
        return (None, None);
    }

    let split_input: Vec<&str> = user_input.split(':').map(|s| s.trim()).collect();

    let mut parsed_details: Vec<String> = Vec::new();
    if split_input.len() > 1 {
        let details = split_input[1..].join(" : ");
        if !details.is_empty() {
            // prevent empty strings
            parsed_details.push(details);
        }
    }

    let shortcut_section = split_input[0];
    let mut parsed_shortcuts = Vec::<i32>::new();
    let mut unknowns_in_shortcut_area = Vec::<String>::new();

    for possible_shortcut in shortcut_section
        .split(' ')
        .map(|s| s.to_lowercase().trim().to_owned())
    {
        if possible_shortcut.is_empty() {
            // Nothing to see here...
            continue;
        }
        let mut found = false;
        for (id, s) in shortcuts.iter() {
            if possible_shortcut == s.to_lowercase() {
                parsed_shortcuts.push(*id);
                found = true;
                break;
            }
        }
        if !found {
            // just add as a text detail, I guess.
            unknowns_in_shortcut_area.push(possible_shortcut);
        }
    }

    if !unknowns_in_shortcut_area.is_empty() {
        parsed_details.push(unknowns_in_shortcut_area.join(" "));
    }

    let mut result: (Option<Vec<i32>>, Option<String>) = (None, None);

    if !parsed_shortcuts.is_empty() {
        result.0 = Some(parsed_shortcuts);
    }
    if !parsed_details.is_empty() {
        result.1 = Some(parsed_details.join("; "));
    }

    result
}

fn printable_entries(
    starting_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
) -> Result<String, crate::errors::Error> {
    let response = backend::api::get_entries_between_dates(starting_date, end_date)?;

    let mut answer = String::new();

    for (date, group) in &response
        .into_iter()
        .group_by(|api::EntryWithLabelsTuple(e, _, _)| e.timestamp.date())
    {
        let mut tmp_str = String::new();

        tmp_str.push_str(format!("## {}\n", &date.to_string()).as_str());

        for (time, group) in &group.group_by(|EntryWithLabelsTuple(e, _, _)| e.timestamp.time()) {
            tmp_str.push_str(format!("### {}\n", time).as_str());
            for api::EntryWithLabelsTuple(entry, category_label, choice_label) in group {
                if let Some(cat) = category_label {
                    tmp_str.push_str(&cat);
                    if let Some(choice) = choice_label {
                        tmp_str.push_str(format!(" -> {}", choice).as_str());
                    }
                }
                if let Some(ref details) = entry.details {
                    if entry.category.is_some() {
                        tmp_str.push_str(" : ");
                    }
                    tmp_str.push_str(details);
                }
                tmp_str.push_str("  \n");
            }
        }
        answer.push_str(&tmp_str);
        answer.push('\n');
        answer.push('\n');
    }

    Ok(answer.trim().to_owned())
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
