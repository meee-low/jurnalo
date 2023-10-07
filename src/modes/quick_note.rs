use crate::backend::api;
use crate::errors::Error;

pub fn parse_note(content: &[String]) -> Result<(), Error> {
    let message = content.join(" ");
    if message.is_empty() {
        println!("You must provide a message with this command.")
    }
    match add_note(message) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn add_note(note: String) -> Result<(), Error> {
    if note.is_empty() {
        todo!();
    }
    api::post_entry(None, None, Some(note.clone()))?;
    println!("Success! Saved note:\n{}", note);
    Ok(())
}
