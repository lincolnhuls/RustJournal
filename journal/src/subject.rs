// Imports for reading and writing files
use serde::{Serialize, Deserialize};
use crate::entry::Entry;
// Imports for reading and writing user input
use std::{io};
// Import for clearing the screen on display calls
use std::process::Command;
// Import for getting current date for the log
use chrono::Local;

#[derive(Serialize, Deserialize)]
pub struct Subject {
    pub entries: Vec<Entry>,
    pub name: String,
}

impl Subject {
    pub fn new() -> Self {
        Subject {
            entries: Vec::new(),
            name: String::new(),
        }
    }

    pub fn display(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
        for entry in &self.entries {
            println!("{}", entry.date);
            println!("{}", entry.log);
            println!("");
        }
    }

       // Add date and text to entry
    pub fn create_entry(&mut self) {
        let now = Local::now();
        let date = now.format("%m-%d-%Y").to_string();
        io::Write::flush(&mut io::stdout()).unwrap();
        println!("Please enter you text here, type /quit/ to finish:");
        let mut lines = Vec::new();
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Please enter your text.");
            let trimmed = input.trim();
            if trimmed == "/quit/" {
                break;
            }
            lines.push(trimmed.to_string());
        }
        let log = lines.join("\n");

        let entry = Entry {
            date, 
            log,
        };
        self.entries.push(entry);
    }
}