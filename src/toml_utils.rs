extern crate dotenvy;
extern crate toml;

// use std::fs;

// fn load_toml() {
//     todo!()
// }

pub mod toml_schema {
    use serde_derive::Deserialize;

    #[derive(Deserialize)]
    pub struct TomlData {
        pub questions: Box<[Question]>,
        pub batteries_of_questions: Box<[BatteryOfQuestionsConfig]>,
    }

    #[derive(Deserialize)]
    pub struct Question {
        pub id: u32,
        pub prompt: String,
        pub options: Box<[QuestionOption]>,
    }

    #[derive(Deserialize)]
    pub struct QuestionOption {
        pub shortcut: String,
        pub label: String,
    }

    #[derive(Deserialize)]
    pub struct BatteryOfQuestionsConfig {
        pub command: String,
        pub questions: Box<[u32]>,
    }
}

#[test]
fn test_load_toml() {
    use toml_schema::TomlData;

    dotenvy::dotenv().ok();

    let toml_path =
        std::env::var("TEST_TOML").expect("Could not find `TEST_TOML` in the environment.");
    dbg!(&toml_path);
    let toml_string = std::fs::read_to_string(&toml_path).expect("Could not read the toml file.");
    let toml_data: TomlData =
        toml::from_str(&toml_string).expect("Failed to parse toml into data.");

    assert!(toml_data.questions.len() > 0);
    assert_eq!(
        toml_data.questions[2].prompt,
        "What habits did you accomplish today?"
    );
    assert_ne!(toml_data.questions[0].prompt, "How was your mood today?");
    assert_eq!(toml_data.questions[2].options[0].shortcut, "M");
}
