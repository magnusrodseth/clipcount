use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use unicode_segmentation::UnicodeSegmentation;

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

    if text.is_empty() {
        eprintln!("No text found on the clipboard");
        return;
    }

    let Processed {
        line_count,
        word_count,
        character_count,
    } = process_text(&text);

    if args.wc {
        println!("{} {} {}", line_count, word_count, character_count);
        return;
    }

    println!("Word count: {}", word_count);
    println!("Character count: {}", character_count);
    println!("Line count: {}", line_count);
}

struct Processed {
    line_count: usize,
    word_count: usize,
    character_count: usize,
}

/// Process the text to count lines, words, and characters.
fn process_text(text: &str) -> Processed {
    let words = text.unicode_words().collect::<Vec<&str>>();
    let word_count = words.len();
    let character_count = text.graphemes(true).count();
    let line_count = text.lines().count();

    Processed {
        line_count,
        word_count,
        character_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_text() {
        let text = "Hello, world!\nThis is a test.\nAnd another line.";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 3);
        assert_eq!(word_count, 9);
        assert_eq!(character_count, 47);
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 0);
        assert_eq!(word_count, 0);
        assert_eq!(character_count, 0);
    }

    #[test]
    fn test_unicode_text() {
        let text = "Hello, 👨‍👩‍👧‍👦!";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 1);
        assert_eq!(word_count, 1);
        assert_eq!(character_count, 9);
    }

    #[test]
    fn test_multilingual_text() {
        let text = "你好, 世界!\nBonjour le monde!\nHello, world!";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 3);
        assert_eq!(word_count, 9);
        assert_eq!(character_count, 39);
    }

    #[test]
    fn test_text_with_multiple_spaces() {
        let text = "This    is a     test text.";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 1);
        assert_eq!(word_count, 5);
        assert_eq!(character_count, 27);
    }

    #[test]
    fn test_text_with_emojis() {
        let text = "🚀 Rocket to the moon! 🌕";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 1);
        assert_eq!(word_count, 4);
        assert_eq!(character_count, 23);
    }

    #[test]
    fn test_newline_characters() {
        let text = "Line one.\nLine two.\nLine three.";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 3);
        assert_eq!(word_count, 6);
        assert_eq!(character_count, 31);
    }

    #[test]
    fn test_text_with_punctuation() {
        let text = "Hello, world! This is a test: How will it handle punctuation?";
        let Processed {
            line_count,
            word_count,
            character_count,
        } = process_text(text);

        assert_eq!(line_count, 1);
        assert_eq!(word_count, 11);
        assert_eq!(character_count, 61);
    }
}
