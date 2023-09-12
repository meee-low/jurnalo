use std::env;
use std::fs::File;
use std::io::Write;

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
        Full => { todo!() }
        Print => { todo!() }
        Export => { todo!() }
    };
    match command_result {
        Ok(_) => Ok(()),
        Err(_) => panic!()
    }
}

fn parse_note(content: &[String]) -> std::io::Result<()> {
    let message = content.join(" ");
    if message.len() == 0 {
        println!("You must provide a message with this command.")
    }
    add_note(message)
}


fn add_note(note: String) -> std::io::Result<()> {
    // Currently just a dummy implementation for testing.
    // TODO: Needs to be connected to database.
    let mut file = File::options().append(true).create(true).open("logs.txt")?;
    writeln!(&mut file, "{}", note)?;
    Ok(())
}
#[derive(Debug)]
enum ParsingCommandError {
    TooFewArguments,
    CommandNotRecognized
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