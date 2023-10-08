use crate::backend::api;
use crate::errors::Error;

pub fn new_category(label: &str, prompt: &str) -> Result<(), Error> {
    if label.is_empty() || prompt.is_empty() {
        return Err(Error::InvalidInput(
            "You must provide a label and a prompt.".to_owned(),
        ));
    }
    api::post_category(label, prompt)?;
    println!("Success! Added category {}.", label);
    Ok(())
}

pub fn new_choice(label: &str, shortcut: &str, category: &str) -> Result<(), Error> {
    if label.is_empty() || shortcut.is_empty() || category.is_empty() {
        return Err(Error::InvalidInput(
            "You must provide a label, a shortcut, and a category.".to_owned(),
        ));
    }
    api::post_choice(label, shortcut, category)?;
    println!("Success! Added choice {}.", label);
    Ok(())
}

pub fn new_quiz(label: &str) -> Result<(), Error> {
    if label.is_empty() {
        return Err(Error::InvalidInput("You must provide a label.".to_owned()));
    }
    api::post_quiz(label)?;
    println!("Success! Added quiz {}.", label);
    Ok(())
}

pub fn link_category_to_quiz(category: &str, quiz: &str) -> Result<(), Error> {
    if category.is_empty() || quiz.is_empty() {
        return Err(Error::InvalidInput(
            "You must provide a category and a quiz.".to_owned(),
        ));
    }
    api::patch::link_category_to_quiz(category, quiz)?;
    println!("Success! Added category {} to quiz {}.", category, quiz);
    Ok(())
}

pub fn unlink_category_from_quiz(category: &str, quiz: &str) -> Result<(), Error> {
    if category.is_empty() || quiz.is_empty() {
        return Err(Error::InvalidInput(
            "You must provide a category and a quiz.".to_owned(),
        ));
    }
    api::patch::unlink_category_from_quiz(category, quiz)?;
    println!("Success! Added category {} to quiz {}.", category, quiz);
    Ok(())
}

pub fn list_all_categories() -> Result<(), Error> {
    let categories = api::get_all_categories()?;
    println!("Categories:");
    for category in categories {
        println!("{}: {}", category.label, category.prompt);
    }
    Ok(())
}

pub fn list_all_categories_in_quiz(quiz: &str) -> Result<(), Error> {
    if quiz.is_empty() {
        return Err(Error::InvalidInput("You must provide a quiz.".to_owned()));
    }
    let categories = api::get_categories_in_quiz(quiz)?;
    println!("Categories in quiz {}:", quiz);
    for category in categories {
        println!("{}: {}", category.label, category.prompt);
    }
    Ok(())
}

pub fn list_all_choices_in_category(category_label: &str) -> Result<(), Error> {
    if category_label.is_empty() {
        return Err(Error::InvalidInput(
            "You must provide a category.".to_owned(),
        ));
    }
    let choices = api::get_choices_in_category(category_label)?;
    println!("Choices in category {}:", category_label);
    for choice in choices {
        println!("{}: {}", choice.label, choice.shortcut);
    }
    Ok(())
}

pub fn disable_choice(choice: &str) -> Result<(), Error> {
    if choice.is_empty() {
        return Err(Error::InvalidInput("You must provide a choice.".to_owned()));
    }
    api::patch::disable_choice(choice)?;
    println!("Success! Disabled choice {}.", choice);
    Ok(())
}

pub fn disable_category(category: &str) -> Result<(), Error> {
    if category.is_empty() {
        return Err(Error::InvalidInput(
            "You must provide a category.".to_owned(),
        ));
    }
    api::patch::disable_category(category)?;
    println!("Success! Disabled category {}.", category);
    Ok(())
}

pub fn toggle_show_in_streaks_for_choice(choice: &str) -> Result<(), Error> {
    if choice.is_empty() {
        return Err(Error::InvalidInput("You must provide a choice.".to_owned()));
    }
    api::patch::toggle_show_in_streaks_for_choice(choice)?;
    println!("Success! Toggled show_in_streaks for choice {}.", choice);
    Ok(())
}

pub fn change_timer_for_choice(choice: &str, new_timer: u32) -> Result<(), Error> {
    if choice.is_empty() {
        return Err(Error::InvalidInput("You must provide a choice.".to_owned()));
    }
    api::patch::change_timer_for_choice(choice, new_timer)?;
    println!(
        "Success! Changed timer for choice {} to {} days.",
        choice, new_timer
    );
    Ok(())
}

pub fn move_last_entry_to_yesterday() -> Result<(), Error> {
    api::patch::move_last_entry_to_yesterday()?;
    println!("Success! Moved last entry to yesterday.");
    Ok(())
}
