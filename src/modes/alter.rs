use core::panic;

/// This module contains functions for altering the database.
use crate::backend::api;

pub fn new_category(label: &str, prompt: &str) {
    if label.is_empty() || prompt.is_empty() {
        panic!("Invalid Input: You must provide a label and a prompt.");
    }
    api::post_category(label, prompt).unwrap();
    println!("Success! Added category {}.", label);
}

pub fn new_choice(label: &str, shortcut: &str, category: &str) {
    if label.is_empty() || shortcut.is_empty() || category.is_empty() {
        panic!("Invalid Input: You must provide a label, a shortcut, and a category.");
    }
    api::post_choice(label, shortcut, category).unwrap();
    println!("Success! Added choice {}.", label);
}

pub fn new_quiz(label: &str) {
    if label.is_empty() {
        panic!("Invalid Input: You must provide a label for the quiz.");
    }
    api::post_quiz(label).unwrap();
    println!("Success! Added quiz {}.", label);
}

pub fn link_category_to_quiz(category: &str, quiz: &str) {
    if category.is_empty() || quiz.is_empty() {
        panic!("Invalid Input: You must provide a category and a quiz.");
    }
    api::patch::link_category_to_quiz(category, quiz).unwrap();
    println!("Success! Added category {} to quiz {}.", category, quiz);
}

pub fn unlink_category_from_quiz(category: &str, quiz: &str) {
    if category.is_empty() || quiz.is_empty() {
        panic!("Invalid Input: You must provide a category and a quiz.");
    }
    api::patch::unlink_category_from_quiz(category, quiz).unwrap();
    println!("Success! Added category {} to quiz {}.", category, quiz);
}

pub fn list_all_categories() {
    let categories = api::get_all_categories().unwrap();
    println!("Categories:");
    for category in categories {
        println!("{}: {}", category.label, category.prompt);
    }
}

pub fn list_all_categories_in_quiz(quiz: &str) {
    if quiz.is_empty() {
        panic!("Invalid Input: You must provide a quiz.");
    }
    let categories = api::get_categories_in_quiz(quiz).unwrap();
    println!("Categories in quiz {}:", quiz);
    for category in categories {
        println!("{}: {}", category.label, category.prompt);
    }
}

pub fn list_all_choices_in_category(category_label: &str) {
    if category_label.is_empty() {
        panic!("Invalid Input: You must provide a category.");
    }
    let choices = api::get_choices_in_category(category_label).unwrap();
    println!("Choices in category {}:", category_label);
    for choice in choices {
        println!("  {}: {}", choice.label, choice.shortcut);
    }
}

pub fn disable_choice(category: &str, choice: &str) {
    if choice.is_empty() || category.is_empty() {
        panic!("Invalid Input: You must provide a choice and a category.");
    }
    api::patch::disable_choice(category, choice).unwrap();
    println!("Success! Disabled choice {}.", choice);
}

pub fn disable_category(category: &str) {
    if category.is_empty() {
        panic!("Invalid Input: You must provide a category.");
    }
    api::patch::disable_category(category).unwrap();
    println!("Success! Disabled category {}.", category);
}

pub fn toggle_show_in_streaks_for_choice(category: &str, choice: &str) {
    if choice.is_empty() {
        panic!("Invalid Input: You must provide a choice.")
    }
    api::patch::toggle_show_in_streaks_for_choice(category, choice).unwrap();
    println!("Success! Toggled show_in_streaks for choice {}.", choice);
}

pub fn change_timer_for_choice(category: &str, choice: &str, new_timer: i32) {
    if choice.is_empty() || category.is_empty() {
        panic!("Invalid Input: You must provide a choice and a category.")
    }
    let timer_arg = if new_timer == -1 {
        None
    } else if new_timer >= 1 {
        Some(new_timer)
    } else {
        panic!("Invalid Input: Timer must be -1 or greater than 1.")
    };

    api::patch::change_timer_for_choice(choice, timer_arg).unwrap();
    println!(
        "Success! Changed timer for choice {} to {} days.",
        choice, new_timer
    );
}

pub fn move_last_entry_to_yesterday() {
    api::patch::move_last_entry_to_yesterday().unwrap();
    println!("Success! Moved last entry to yesterday.");
}
