use std::env;
use std::fs::File;
use std::io::Write;

use json::JsonValue;

const MOCK_LOG_PATH: &str = "mockdb/logs.txt";
const MOCK_SETTINGS_PATH: &str = "mockdb/settings.json";

fn main() -> Result<(), IOorParsingError> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => { assert!(false, "Unreachable.") }
        1 => {
            println!("Insufficient arguments");
            // TODO: Help message. Only returning error for now.
            return Err(IOorParsingError::Parsing(ParsingCommandError::TooFewArguments)) }
        _ => {
            // Has command.
            parse_command(&args[1],&args[2..])?;
        }
    }
    Ok(())
}

fn parse_command(command_string: &String, content: &[String]) -> Result<(), IOorParsingError> {
    // Keep content as array because it's useful for some commands
    // that may take multiple commands (e.g.: `jurnalo habit add)
    use Command::*;
    let command = Command::from_string(&command_string).expect("Command not recognized.");
    let command_result = match command {
        QuickNote => { parse_note(content) }
        Habit => { todo!() }
        Full => { full_battery(content) }
        Print => { todo!() }
        Export => { todo!() }
    };
    match command_result {
        Ok(_) => Ok(()),
        Err(_) => panic!()
    }
}
fn parse_note(content: &[String]) -> Result<(), IOorParsingError> {
    let message = content.join(" ");
    if message.len() == 0 {
        println!("You must provide a message with this command.")
    }
    match add_note(message) {
        Ok(_) => Ok(()),
        Err(e) => Err(IOorParsingError::IO(e))
    }
}


fn add_note(note: String) -> std::io::Result<()> {
    // Currently just a dummy implementation for testing.
    // TODO: Needs to be connected to database.
    let mut file = File::options().append(true).create(true).open(MOCK_LOG_PATH)?;
    writeln!(&mut file, "{}", note)?;
    Ok(())
}

fn full_battery(content: &[String]) -> Result<(), IOorParsingError> {
    // TODO: Idea: Maybe allow for variants of this? Probe the database for the keyword.
    // E.g.: jurnalo full day, jurnalo full week, jurnalo full social, ...
    // Then it's probably a good idea to just have the command itself (jurnalo day, jurnalo week, ...)
    // For now, complaining when there are extra words.
    if content.len() != 0 { return Err(IOorParsingError::Parsing(ParsingCommandError::TooManyArguments)); }

    let settings_raw = std::fs::read_to_string(MOCK_SETTINGS_PATH).expect("Couldn't open the settings JSON.");
    let settings = json::parse(&settings_raw).expect("Couldn't parse the JSON for questions.");

    // TODO: factor out this type-based value extraction. Dunno if we're even using JSON for serialization, so change nothing for now.
    let questions: Vec<JsonValue> = match get_key_from_object(&settings, "questions") {
        JsonValue::Array(q) => q,
        _ => panic!("`questions` is not a JSON Array.")
    };

    let mut inputs: Vec<String> = Vec::new();

    for question in questions.iter() {
        // TODO: Make the order as specified by the battery.
        let prompt: String = match get_key_from_object(question, "prompt") {
            json::JsonValue::String(prompt_string) => {prompt_string},
            json::JsonValue::Short(prompt_string) => {prompt_string.as_str().to_owned()}, // TODO: use refs/slices
            _ => {
                dbg!(question);
                panic!("`prompt` is not a JSON String.")
            }
        };

        let options: Vec<JsonValue> = match get_key_from_object(question, "options") {
            json::JsonValue::Array(vec_values) => {vec_values},
            _ => panic!("`categories` is not a JSON Array.")
        };


        let mut shortcut_label_pairs: Vec<(String, String)> = Vec::new();
        for option in options.iter() {
            let shortcut = match get_key_from_object(option, "shortcut") {
                json::JsonValue::String(shortcut) => shortcut,
                json::JsonValue::Short(shortcut) => {shortcut.as_str().to_owned()}, // TODO: use refs/slices
                _ => panic!("`shortcut` is not a JSON String.")
            };
            let label = match get_key_from_object(option, "label") {
                json::JsonValue::String(label) => label,
                json::JsonValue::Short(label) => {label.as_str().to_owned()}, // TODO: use refs/slices
                _ => panic!("`shortcut` is not a JSON String.")
            };
            shortcut_label_pairs.push((shortcut, label));
        }

        // Display prompt
        println!("{}", prompt);
        println!("{}", shortcut_label_pairs.iter().map(|(s, v)| format!("[{}] {}", s, v)).collect::<Vec<String>>().join(" "));

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
            }
            Err(_) => {
                panic!("Couldn't handle user input.")
            }
        }
        inputs.push(input);
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
    Parsing(ParsingCommandError)
}

enum Command {
    Full,
    QuickNote,
    Habit,
    Print,
    Export,
}

impl Command {
    fn from_string(command_string: &String) -> Result<Self, ParsingCommandError> {
        match command_string.to_lowercase().as_str() {
            "full" => Ok(Command::Full),
            "note" => Ok(Command::QuickNote),
            "habit" => Ok(Command::Habit),
            "print" => Ok(Command::Print),
            "export" => Ok(Command::Export),
            _ => Err(ParsingCommandError::CommandNotRecognized)
        }
    }
}


fn get_key_from_object(possible_object: &json::JsonValue , key: &str) -> JsonValue {
    match possible_object {
        JsonValue::Object(keys) => {
            match keys.get(key){
                Some(content) => {
                    content.to_owned()
                }
                None => panic!("Key {} not found in the object.", key)
            }
        }
        _ => panic!("This is not a JSON object.")
    }

}