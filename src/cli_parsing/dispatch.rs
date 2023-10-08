use super::clap_structs::{Args, CategorySubcommands, QuizSubcommands, SubCommand};
use crate::errors::Error;

use crate::modes;

pub fn dispatch(args: &Args) -> Result<(), Error> {
    if args.quiz.is_none() && args.note.is_none() && args.subcommand.is_none() {
        println!("No arguments provided");
    }

    // From now on, we can assume that at least one of the arguments is Some.
    // Since quiz and note are positional arguments, we can assume note is None if quiz is None.
    // Therefore, at least one of quiz or subcommand is Some.

    if let Some(ref quiz_name) = args.quiz {
        modes::run_quiz::quiz_full(quiz_name);
        return Ok(());
    }

    // Now we can assume that quiz and note are None, and subcommand is Some.

    match args.subcommand {
        Some(ref subcommand) => match subcommand {
            SubCommand::Print { days, output } => {
                println!("Print: {:?}", subcommand);
                todo!();
            }
            SubCommand::Category { subcommand } => {
                dispatch_category_subcommands(subcommand);
            }
            SubCommand::Quiz { subcommand } => {
                dispatch_quiz_subcommands(subcommand);
            }
            SubCommand::Init { path, config } => {
                println!("Init: {:?}", subcommand);
                todo!();
            }
        },
        None => {
            unreachable!();
        }
    }

    todo!("{:?}", args);
}

fn dispatch_category_subcommands(subcommand: &CategorySubcommands) {
    match subcommand {
        CategorySubcommands::Create { category, prompt } => {
            println!("Category: {:?}", subcommand);
            todo!();
        }
        CategorySubcommands::Disable { category } => {
            println!("Category: {:?}", subcommand);
            todo!();
        }
        CategorySubcommands::Rename { category, new_name } => {
            println!("Category: {:?}", subcommand);
            todo!();
        }
        CategorySubcommands::List => {
            println!("Category: {:?}", subcommand);
            todo!();
        }
        CategorySubcommands::AddChoice {
            category,
            choice_label,
            choice_shortcut,
        } => {
            println!("Category: {:?}", subcommand);
            todo!();
        }
        CategorySubcommands::ListChoices { category } => {
            println!("Category: {:?}", subcommand);
            todo!();
        }
    }
}

fn dispatch_quiz_subcommands(subcommand: &QuizSubcommands) {
    match subcommand {
        QuizSubcommands::Create { quiz } => {
            println!("Quiz: {:?}", subcommand);
            todo!();
        }
        QuizSubcommands::AddCategory { quiz, category } => {
            println!("Quiz: {:?}", subcommand);
            todo!();
        }
        QuizSubcommands::RemoveCategory { quiz, category } => {
            println!("Quiz: {:?}", subcommand);
            todo!();
        }
        QuizSubcommands::Rename { quiz, new_name } => {
            println!("Quiz: {:?}", subcommand);
            todo!();
        }
        QuizSubcommands::List => {
            println!("Quiz: {:?}", subcommand);
            todo!();
        }
    }
}
