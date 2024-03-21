use colored::Colorize;
use similar::{ChangeTag, TextDiff};

fn main() {
    let diff = TextDiff::from_lines(
        "Hello World\nThis is the second line.\nThis is the third.",
        "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more",
    );

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => print!("{} {}", "▎".red(), change.to_string().red()),
            ChangeTag::Insert => print!("{} {}", "▎".green(), change.to_string().green()),
            ChangeTag::Equal => print!("  {}", change),
        };
    }
}
