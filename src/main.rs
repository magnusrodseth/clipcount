use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

/// Counts words, characters and lines from the clipboard content.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Output in `wc` command format: line count, word count, character count
    #[clap(long, action = clap::ArgAction::SetTrue)]
    wc: bool,
}

fn main() {
    let args = Args::parse();

    // Read whatever is on the clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let text = match ctx.get_contents() {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    // If nothing is on the clipboard, print a message
    if text.is_empty() {
        eprintln!("No text found on the clipboard");
        return;
    }

    // Count the number of words
    let word_count = text.split_whitespace().count();
    let character_count = text.chars().count();
    let line_count = text.lines().count();

    if args.wc {
        println!("{} {} {}", line_count, word_count, character_count);
        return;
    }

    println!("Word count: {}", word_count);
    println!("Character count: {}", character_count);
    println!("Line count: {}", line_count);
}
