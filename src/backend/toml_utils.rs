extern crate dotenvy;
extern crate toml;

use std::path::Path;

use toml_schema::TomlData;

pub fn load_toml(path_string: &str) -> Result<TomlData, toml::de::Error> {
    let mut test_toml_path_string = path_string.to_owned();
    dotenvy::dotenv().ok();
    let maybe_test_toml = std::env::var("TEST_TOML");

    if path_string.is_empty() {
        test_toml_path_string =
            maybe_test_toml.expect("Could not find `TEST_TOML` in the environment.");
    }
    let toml_path = Path::new(test_toml_path_string.as_str());

    let toml_string = std::fs::read_to_string(toml_path).unwrap_or(format!(
        "Could not read the toml file at path: {:#?}.",
        toml_path
    ));
    // TODO: Validate TOML (make sure the foreign keys are valid (I think it's only on the quiz section).)
    toml::from_str::<TomlData>(&toml_string)
}

pub mod toml_schema {
    use serde_derive::Deserialize;

    #[derive(Deserialize)]
    pub struct TomlData {
        pub categories: Box<[Category]>,
        pub quizzes: Box<[Quiz]>,
    }

    #[derive(Deserialize)]
    pub struct Category {
        pub label: String,
        pub prompt: String,
        pub choices: Option<Box<[Choice]>>,
        pub question_type: Option<i32>,
        pub extra_info: Option<String>,
        pub show_in_streaks: Option<i32>,
        pub reminder_timer_in_days: Option<i32>,
    }

    #[derive(Deserialize)]
    pub struct Choice {
        pub shortcut: String,
        pub label: String,
        pub show_in_streaks: Option<i32>,
        pub reminder_timer_in_days: Option<i32>,
    }

    #[derive(Deserialize)]
    pub struct Quiz {
        pub command: String,
        pub categories: Box<[String]>,
    }
}

#[test]
fn test_load_toml() {
    let toml_data = load_toml("").expect("Couldn't parse the test TOML.");

    assert!(toml_data.categories.len() > 0);
    assert_eq!(
        toml_data.categories[2].prompt,
        "What habits did you accomplish today?"
    );
    assert_ne!(toml_data.categories[0].prompt, "How was your mood today?");
    // assert_eq!(toml_data.categories[2].choices.unwrap()[0].shortcut, "M");
    assert_eq!(toml_data.categories[1].question_type, None);
}
