// Imports for reading and writing user input
use std::{io};
// Imports for reading and writing to files
use serde_json;
use std::fs;
use std::fs::File;
use std::io::Write;

mod entry;
mod subject;
mod journal;

fn save_journal(journal: &journal::Journal, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(journal).unwrap();
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_journal(path: &str) -> std::io::Result<journal::Journal> {
    let data = fs::read_to_string(path)?;
    let journal: journal::Journal = serde_json::from_str(&data).unwrap();
    Ok(journal)
}

fn main() {
    let mut journal = journal::Journal::new();

    println!("Welcome to the Journal!");
    loop {
        println!("1. Save Journal");
        println!("2. Load Journal");
        println!("3. Display Subjects");
        println!("4. Create Subject");
        println!("5. Select Subject");
        println!("6. Quit");
        print!("Choose an option: ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Please enter a choice");
        let trimmed_choice = choice.trim().to_lowercase();
        match trimmed_choice.as_str() {
            "1" => {
                match save_journal(&journal, "journal.json") {
                    Ok(_) => println!("Journal saved successfully!"),
                    Err(e) => println!("Failed to save: {}", e),
                }
            },
            "2" => {
                match load_journal("journal.json") {
                    Ok(j) => journal = j,
                    Err(e) => println!("Failed to load: {}", e),
                }
            },
            "3" => journal.display(),
            "4" => journal.create_subject(),
            "5" => journal.select_subject(),
            "6" => break,
            _ => println!("Invalid option. Please try again."),
        }
    }
}
