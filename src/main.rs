use std::{
    collections::BTreeMap,
    io::{self, BufRead}
};

use colored::Colorize;
use similar::{ChangeTag, TextDiff};

#[derive(Debug)]
struct Line {
    old: Option<(ChangeTag, String)>,
    new: Option<(ChangeTag, String)>
}

// A pad function that fills the string with spaces to the desired width or cuts it if it's too long
fn pad(s: String, width: usize) -> String {
    if s.len() > width {
        s.chars().take(width - 4).collect::<String>() + "... "
    } else {
        s.clone() + &" ".repeat(width - s.len())
    }
}

fn main() {
    let width = termsize::get().map(|size| size.cols).unwrap() as usize;

    let stdin = io::stdin();
    let mut first: Vec<String> = Vec::new();
    let mut second: Vec<String> = Vec::new();

    let text = "Enter the first text, then type 'EOF' and press enter: ";
    println!("{}{}", text.dimmed(), "-".repeat(width - text.len()));
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "EOF" {
            break;
        }
        first.push(line);
    }

    let text = "Enter the second text, then type 'EOF' and press enter: ";
    println!("{}{}", text.dimmed(), "-".repeat(width - text.len()));
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "EOF" {
            break;
        }
        second.push(line);
    }
    println!("{}", "-".repeat(width)); // Separator

    let first = first.join("\n");
    let second = second.join("\n");
    let diff = TextDiff::from_lines(first.as_str(), second.as_str());

    let mut lines: BTreeMap<i16, Line> = BTreeMap::new();
    diff.iter_all_changes().for_each(|change| {
        let (tag, value) = match change.tag() {
            ChangeTag::Delete => (ChangeTag::Delete, change.value().replace("\n", "")),
            ChangeTag::Insert => (ChangeTag::Insert, change.value().replace("\n", "")),
            ChangeTag::Equal => (ChangeTag::Equal, change.value().replace("\n", ""))
        };

        if change.old_index().is_some() && change.new_index().is_some() {
            let old_index = change.old_index().unwrap() as i16;
            lines.insert(
                old_index,
                Line {
                    old: Some((tag, value.clone())),
                    new: Some((tag, value))
                }
            );
        } else if change.old_index().is_some() {
            let old_index = change.old_index().unwrap() as i16;
            if lines.get(&old_index).is_some() {
                lines.get_mut(&old_index).unwrap().old = Some((tag, value));
            } else {
                lines.insert(
                    old_index,
                    Line {
                        old: Some((tag, value)),
                        new: None
                    }
                );
            }
        } else {
            let new_index = change.new_index().unwrap() as i16;
            if lines.get(&new_index).is_some() {
                lines.get_mut(&new_index).unwrap().new = Some((tag, value));
            } else {
                lines.insert(
                    new_index,
                    Line {
                        old: None,
                        new: Some((tag, value))
                    }
                );
            }
        }
    });

    for (index, line) in lines.iter() {
        let print = {
            let old = line
                .old
                .clone()
                .unwrap_or((ChangeTag::Equal, "".to_string()));

            let new = line
                .new
                .clone()
                .unwrap_or((ChangeTag::Equal, "".to_string()));

            let line_number = format!("{:<3}", index + 1);

            let old_line = pad(old.1, (width - 12) / 2);
            let new_line = pad(new.1, (width - 12) / 2);

            let old_line = format!("{} ┃ {}", line_number, old_line);
            let new_line = format!("{} ┃ {}", line_number, new_line);

            format!(
                "{}{}",
                match old.0 {
                    ChangeTag::Delete => old_line.red(),
                    ChangeTag::Insert => old_line.green(),
                    ChangeTag::Equal => old_line.dimmed()
                },
                match new.0 {
                    ChangeTag::Delete => new_line.red(),
                    ChangeTag::Insert => new_line.green(),
                    ChangeTag::Equal => new_line.dimmed()
                },
            )
        };

        println!("{}", print);
    }
}
