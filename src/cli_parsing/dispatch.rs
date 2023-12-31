use super::clap_structs::{
    Args, CategorySubcommands, ChoiceSubcommands, EntriesSubcommands, QuizSubcommands, SubCommand,
};

use crate::modes;

/// Dispatches the arguments to the appropriate functions.
pub fn dispatch(args: &Args) {
    if args.quiz.is_none() && args.note.is_none() && args.subcommand.is_none() {
        println!("No arguments provided");
    }

    // From now on, we can assume that at least one of the arguments is Some.
    // Since quiz and note are positional arguments, we can assume note is None if quiz is None.
    // Therefore, at least one of quiz or subcommand is Some.

    if let Some(ref quiz_name) = args.quiz {
        modes::run_quiz::quiz_full(quiz_name).unwrap();
        return;
    }

    // Now we can assume that quiz and note are None, and subcommand is Some.

    match args.subcommand {
        Some(ref subcommand) => match subcommand {
            SubCommand::Print { days, output } => {
                modes::print::print(*days, output);
            }
            SubCommand::Category { subcommand } => {
                dispatch_category_subcommands(subcommand);
            }
            SubCommand::Quiz { subcommand } => {
                dispatch_quiz_subcommands(subcommand);
            }
            SubCommand::Choice { subcommand } => dispatch_choice_subcommands(subcommand),
            SubCommand::Entries { subcommand } => {
                dispatch_entries_subcommands(subcommand);
            }
            SubCommand::Init { path, config } => crate::backend::setup(config, path),
        },
        None => {
            unreachable!("If we got here, it means that quiz is None, note is None, and subcommand is None, which goes against our assumptions.");
        }
    }
}

/// Dispatches the category subcommands to the appropriate functions.
fn dispatch_category_subcommands(subcommand: &CategorySubcommands) {
    match subcommand {
        CategorySubcommands::Create { category, prompt } => {
            modes::alter::new_category(category, prompt);
        }
        CategorySubcommands::Disable { category } => {
            modes::alter::disable_category(category);
        }
        CategorySubcommands::Rename { category, new_name } => {
            modes::alter::rename_category(category, new_name);
        }
        CategorySubcommands::List => {
            modes::alter::list_all_categories();
        }
        CategorySubcommands::AddChoice {
            category,
            choice_label,
            choice_shortcut,
        } => {
            modes::alter::new_choice(choice_label, choice_shortcut, category);
        }
        CategorySubcommands::ListChoices { category } => {
            modes::alter::list_all_choices_in_category(category);
        }
    }
}

/// Dispatches the quiz subcommands to the appropriate functions.
fn dispatch_quiz_subcommands(subcommand: &QuizSubcommands) {
    match subcommand {
        QuizSubcommands::Create { quiz } => {
            modes::alter::new_quiz(quiz);
        }
        QuizSubcommands::LinkCategory { quiz, category } => {
            modes::alter::link_category_to_quiz(category, quiz);
        }
        QuizSubcommands::UnlinkCategory { quiz, category } => {
            modes::alter::unlink_category_from_quiz(category, quiz);
        }
        QuizSubcommands::Rename { quiz, new_name } => {
            modes::alter::rename_quiz(quiz, new_name);
        }
        QuizSubcommands::List => {
            modes::alter::list_all_quizzes();
        }
        QuizSubcommands::ListCategories { quiz } => {
            modes::alter::list_all_categories_in_quiz(quiz);
        }
    }
}

fn dispatch_choice_subcommands(subcommand: &ChoiceSubcommands) {
    match subcommand {
        ChoiceSubcommands::Add {
            label,
            shortcut,
            category,
        } => {
            modes::alter::new_choice(label, shortcut, category);
        }
        ChoiceSubcommands::Disable { category, label } => {
            modes::alter::disable_choice(category, label);
        }
        ChoiceSubcommands::Rename {
            category,
            label,
            new_name,
        } => {
            modes::alter::rename_choice(category, label, new_name);
        }
        ChoiceSubcommands::List { category } => {
            modes::alter::list_all_choices_in_category(category);
        }
        ChoiceSubcommands::ChangeTimer {
            category,
            label,
            timer,
        } => {
            modes::alter::change_timer_for_choice(category, label, *timer);
        }
        ChoiceSubcommands::ToggleStreaks { category, label } => {
            modes::alter::toggle_show_in_streaks_for_choice(category, label);
        }
    }
}

fn dispatch_entries_subcommands(subcommand: &EntriesSubcommands) {
    match subcommand {
        EntriesSubcommands::Print { days, output } => {
            modes::print::print(*days, output);
        }
        EntriesSubcommands::PushLatestToYesterday => {
            modes::alter::move_last_entry_to_yesterday();
        }
    }
}
