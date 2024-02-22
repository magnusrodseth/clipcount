extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    // Read whatever is on the clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let text = match ctx.get_contents() {
        Ok(text) => text,
        Err(_) => String::from(""),
    };

    // If nothing is on the clipboard, print a message
    if text.is_empty() {
        println!("Nothing on the clipboard");
        return;
    }

    // Count the number of words
    let word_count = text.split_whitespace().count();
    let character_count = text.chars().count();

    println!("Word count: {}", word_count);
    println!("Character count: {}", character_count);
}
