use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None, name = "jurnalo")]
#[command(arg_required_else_help = true)]
pub struct Args {
    /// The quiz to run.
    pub quiz: Option<String>,
    /// If the quiz is quicknote, the note to add.
    pub note: Option<String>,

    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

// TODO: Remove Clone derive once we fully implement the dispatch function.

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// Prints the recent entries to the terminal.
    Print {
        /// The number of days to print.
        #[arg(short, long, value_name = "DAYS", default_value_t = 7 as u32)]
        days: u32,

        /// Output file: if provided, the output will be written to this file instead of stdout.
        /// If the file already exists, it will be overwritten.
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,
    },

    /// Used for seeing, adding or editing categories of questions.
    Category {
        #[command(subcommand)]
        subcommand: CategorySubcommands,
    },

    /// Used for seeing, adding or editing quizzes.
    Quiz {
        #[command(subcommand)]
        subcommand: QuizSubcommands,
    },
    Init {
        /// The path to the directory where the database will be stored.
        #[arg(short, long, value_name = "PATH")]
        path: Option<String>,

        /// The path to the toml file containing the configuration.
        #[arg(short, long, value_name = "PATH")]
        config: Option<String>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum CategorySubcommands {
    /// Adds a new category.
    Create {
        /// The unique label that will be used internally for the category.
        category: String,

        /// The prompt for the category.
        #[arg(short, long, value_name = "PROMPT")]
        prompt: String,
    },
    /// Lists all categories.
    List,
    /// Edits a category.
    Rename {
        category: String,

        #[arg(short, long, value_name = "NEW_NAME")]
        new_name: String,
    },
    /// Disable a category.
    Disable { category: String },
    /// Add a new choice to category.
    AddChoice {
        category: String,

        #[arg(short, long, value_name = "CHOICE")]
        choice_label: String,

        #[arg(short, long, value_name = "SHORTCUT")]
        choice_shortcut: String, // TODO: Make this optional once we implement auto-shortcuts.
    },
    /// List all choices for the provided category.
    ListChoices { category: String },
}

#[derive(Subcommand, Debug, Clone)]
pub enum QuizSubcommands {
    /// Adds a new quiz.
    Create {
        /// The unique label you'll use to run the quiz.
        quiz: String,
    },
    /// Lists all quizzes.
    List,
    /// Renames a quiz.
    Rename {
        quiz: String,

        #[arg(short, long, value_name = "NEW_NAME")]
        new_name: String,
    },
    /// Add an existing category to the quiz.
    AddCategory {
        quiz: String,

        #[arg(short, long, value_name = "CATEGORY")]
        category: String,
    },
    /// Remove a category from the quiz.
    RemoveCategory {
        quiz: String,

        #[arg(short, long, value_name = "CATEGORY")]
        category: String,
    },
}
