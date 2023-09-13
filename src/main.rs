use std::env;
use std::fs::File;
use std::io::Write;

use json::JsonValue;

const MOCK_LOG_PATH: &str = "mockdb/logs.txt";
const MOCK_SETTINGS_PATH: &str = "mockdb/settings.json";

fn main() -> Result<(), IOorParsingError> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => {
            unreachable!()
        }
        1 => {
            println!("Insufficient arguments");
            // TODO: Help message. Only returning error for now.
            return Err(IOorParsingError::Parsing(
                ParsingCommandError::TooFewArguments,
            ));
        }
        _ => {
            // Has command.
            parse_command(&args[1], &args[2..])?;
        }
    }
    Ok(())
}

fn parse_command(command_string: &str, content: &[String]) -> Result<(), IOorParsingError> {
    // Keep content as array because it's useful for some commands
    // that may take multiple commands (e.g.: `jurnalo habit add)
    use Command::*;
    let command = Command::from_string(command_string).expect("Command not recognized.");
    // TODO: Idea: have the Command take in the content and parse it.
    // Could be useful for multi-word commands e.g.: `jurnalo category add`.
    let command_result = match command {
        QuickNote => parse_note(content),
        Habit => {
            todo!()
        }
        Full => full_battery(content),
        Print => {
            todo!()
        }
        Export => {
            todo!()
        }
    };
    match command_result {
        Ok(_) => Ok(()),
        Err(_) => panic!(),
    }
}

fn parse_note(content: &[String]) -> Result<(), IOorParsingError> {
    let message = content.join(" ");
    if message.is_empty() {
        println!("You must provide a message with this command.")
    }
    match add_note(message) {
        Ok(_) => Ok(()),
        Err(e) => Err(IOorParsingError::IO(e)),
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

fn full_battery(content: &[String]) -> Result<(), IOorParsingError> {
    // TODO: Idea: Maybe allow for variants of this? Probe the database for the keyword.
    // E.g.: jurnalo full day, jurnalo full week, jurnalo full social, ...
    // Then it's probably a good idea to just have the command itself (jurnalo day, jurnalo week, ...)
    // For now, complaining when there are extra words.
    if !content.is_empty() {
        return Err(IOorParsingError::Parsing(
            ParsingCommandError::TooManyArguments,
        ));
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

// TODO: Make these a single kind of error.
#[derive(Debug)]
enum ParsingCommandError {
    TooFewArguments,
    CommandNotRecognized,
    TooManyArguments,
}

#[derive(Debug)]
enum IOorParsingError {
    IO(std::io::Error),
    Parsing(ParsingCommandError),
    IncompatibleJSONType,
}

// impl From<T> for ParsingCommandError {}

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

fn get_key_from_object<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Option<&'a JsonValue> {
    match possible_object {
        JsonValue::Object(keys) => keys.get(key),
        _ => None,
    }
}

fn get_key_from_object_as_str<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Result<Option<&'a str>, IOorParsingError> {
    match get_key_from_object(possible_object, key) {
        Some(v) => match v {
            JsonValue::Short(s) => Ok(Some(s.as_str())),
            JsonValue::String(s) => Ok(Some(s)),
            _ => Err(IOorParsingError::IncompatibleJSONType),
        },
        None => Ok(None),
    }
}

fn get_key_from_object_as_vec<'a>(
    possible_object: &'a json::JsonValue,
    key: &str,
) -> Result<Option<&'a Vec<JsonValue>>, IOorParsingError> {
    match get_key_from_object(possible_object, key) {
        Some(v) => match v {
            JsonValue::Array(s) => Ok(Some(s)),
            _ => Err(IOorParsingError::IncompatibleJSONType),
        },
        None => Ok(None),
    }
}
