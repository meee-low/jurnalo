extern crate dotenvy;
extern crate toml;

use toml_schema::TomlData;

pub fn load_toml(path: &str) -> Result<TomlData, toml::de::Error> {
    let toml_path = if path.is_empty() {
        dotenvy::dotenv().ok();
        std::env::var("TEST_TOML").expect("Could not find `TEST_TOML` in the environment.")
    } else {
        path.to_owned()
    };
    let toml_string = std::fs::read_to_string(toml_path).expect("Could not read the toml file.");
    toml::from_str::<TomlData>(&toml_string)
}

pub mod toml_schema {
    use serde_derive::Deserialize;

    #[derive(Deserialize)]
    pub struct TomlData {
        pub questions: Box<[Question]>,
        pub batteries_of_questions: Box<[BatteryOfQuestionsConfig]>,
    }

    #[derive(Deserialize)]
    pub struct Question {
        pub id: i32,
        pub label: String,
        pub prompt: String,
        pub options: Box<[QuestionOption]>,
        pub question_type: Option<i32>,
        pub extra_info: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct QuestionOption {
        pub shortcut: String,
        pub label: String,
    }

    #[derive(Deserialize)]
    pub struct BatteryOfQuestionsConfig {
        pub command: String,
        pub questions: Box<[i32]>,
    }
}

#[test]
fn test_load_toml() {
    let toml_data = load_toml("").expect("Couldn't parse the test TOML.");

    assert!(toml_data.questions.len() > 0);
    assert_eq!(
        toml_data.questions[2].prompt,
        "What habits did you accomplish today?"
    );
    assert_ne!(toml_data.questions[0].prompt, "How was your mood today?");
    assert_eq!(toml_data.questions[2].options[0].shortcut, "M");
    assert_eq!(toml_data.questions[1].question_type, None);
}
