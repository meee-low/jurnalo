use crate::backend::api;
use crate::errors::{Error, ParsingCommandError};
use crate::models;

use chrono::prelude::*;
use std::collections::HashMap;

const STREAK_RANGE: usize = 7; // the last 7 days are used for streaks.

fn get_user_input() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => {
            panic!("Couldn't handle user input.")
        }
    }
}

pub fn quiz_full(quiz_name: &str) -> Result<(), Error> {
    assert!(
        !quiz_name.is_empty(),
        "This should never be empty because we're supposed to parse and handle before this point."
    );
    let mut inputs = Vec::new();
    let mut entries: Vec<(Option<i32>, Option<i32>, Option<String>)> = Vec::new();

    let categories_and_choices = api::get_categories_and_choices_from_quiz_label(quiz_name)?;

    if categories_and_choices.is_empty() {
        return Err(ParsingCommandError::QuizNotFound(quiz_name.to_owned()).into());
    }

    for (cat, choices) in categories_and_choices.iter() {
        println!("{}", cat.prompt);
        if let Some(cs) = choices {
            println!(
                "{}",
                cs.iter()
                    .map(format_choice_and_shortcut)
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
    api::post_multiple_entries(entries).expect("Failed to add to the database.");

    // Print streaks table.
    if let Some(streaks_table) = format_streaks_into_table(fetch_and_process_streaks()) {
        println!("{}", streaks_table);
    }
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

fn fetch_and_process_streaks() -> HashMap<String, [bool; STREAK_RANGE]> {
    use chrono::Duration;

    let today = Local::now().naive_utc();
    let last_seven_days = (0..STREAK_RANGE)
        .map(|offset| (today - Duration::days(offset as i64)).date())
        .collect::<Vec<NaiveDate>>();

    // Initialize the labels and booleans per day
    let mut labels_and_bools: HashMap<String, [bool; STREAK_RANGE]> = HashMap::new();

    let response = api::get_timestamps_for_streaks_of_choices().unwrap();

    for (label, timestamp) in response {
        match timestamp {
            Some(ts) => {
                if last_seven_days.contains(&ts.date()) {
                    // Calculate the day index (0 for today, 6 for 6 days ago)
                    // IDEA: Currently uses difference from the current time. It could be good to use
                    let day_index = (today - ts).num_days() as usize;
                    // Update the boolean array for the label and day
                    let bool_array = labels_and_bools
                        .entry(label.clone())
                        .or_insert([false; STREAK_RANGE]);
                    if day_index <= bool_array.len() {
                        bool_array[day_index] = true;
                    } else {
                        log::warn!("This should only be dealing with entries/timestamps within the streak range.")
                    }
                }
            }
            None => {
                labels_and_bools
                    .entry(label.clone())
                    .or_insert([false; STREAK_RANGE]);
            }
        }
    }
    labels_and_bools
}

fn format_streaks_into_table(
    labels_and_bools: HashMap<String, [bool; STREAK_RANGE]>,
) -> Option<String> {
    if labels_and_bools.is_empty() {
        return None;
    }

    // let days = ["6", "5", "4", "3", "2", "1", "0"];
    let days = (0..STREAK_RANGE)
        .rev()
        .map(|n| format!("{}", n))
        .collect::<Vec<_>>();

    debug_assert!(
        days.iter().all(|d| d.len() == 1),
        "The current formatting expects 1-character columns for days."
    );

    let mut table = String::new();

    // First column padding
    let padding_length = labels_and_bools
        .keys()
        .map(|l| l.len())
        .max()
        .expect("This shouldn't happen because at this point we should always have at least one item in the hashmap.")
        + 2;

    // First column padding for the first row
    for _ in 0..padding_length {
        table += " ";
    }
    for day in days {
        table += &day;
        table += " ";
    }
    table += "\n";

    for (label, bool_array) in labels_and_bools.iter() {
        table += &format!("{:<padding_length$}", label);

        table += &bool_array
            .iter()
            .rev()
            .map(|b| if *b { "# " } else { "  " })
            .collect::<Vec<_>>()
            .concat();
        table += "\n";
    }

    Some(table)
}

fn format_choice_and_shortcut(choice: &models::queryable_or_selectable::Choice) -> String {
    if !choice_is_due(choice) {
        format!("[{}] {}", choice.shortcut, choice.label)
    } else {
        format!("[{}] *{}*", choice.shortcut, choice.label)
    }
}

fn choice_is_due(choice: &models::queryable_or_selectable::Choice) -> bool {
    if choice.reminder_timer_in_days.is_none() {
        return false;
    }

    let reminder_timer_in_days = choice.reminder_timer_in_days.unwrap();
    let now = chrono::Utc::now().naive_utc();
    let timestamp = api::get_latest_timestamp_for_choice(choice.id).unwrap();

    if let Some(ts) = timestamp {
        let days_since_last_entry = (now - ts).num_days();
        return days_since_last_entry >= reminder_timer_in_days.into();
    }

    true // default behavior, if no entry is found
}
