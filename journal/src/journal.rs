// Imports for reading and writing files
use serde::{Serialize, Deserialize};
use crate::subject::Subject;
// Imports for reading and writing user input
use std::{io};
// Import for clearing the screen on display calls
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Journal {
    pub subjects: Vec<Subject>,
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            subjects: Vec::new(),
        }
    }

    pub fn display(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
        for subject in &self.subjects {
            println!("{}", subject.name);
        }
    }

       // Add date and text to entry
    pub fn create_subject(&mut self) {
        print!("Please enter the subject: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut subject_input = String::new();
        io::stdin().read_line(&mut subject_input).expect("Enter a subject.");
        let name = subject_input.trim().to_lowercase();
        let subject = Subject {
            name,
            entries: Vec::new(),
        };
        self.subjects.push(subject);
    }

    pub fn select_subject(&mut self) {
        print!("Please select a subject: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Please enter a subject.");
        let name = input.trim().to_lowercase();

        if let Some(subject) = self.subjects.iter_mut().find(|s| s.name == name) {
            loop {
                println!("\nSubject: {}", subject.name);
                println!("1. View entries");
                println!("2. Create new entry");
                println!("3. Back to main menu");
                print!("Choose an option: ");
                io::Write::flush(&mut io::stdout()).unwrap();

                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("Please enter a choice");
                let trimmed_choice = choice.trim().to_lowercase();
                match trimmed_choice.as_str() {
                    "1" => subject.display(),
                    "2" => subject.create_entry(),
                    "3" => break,
                    _ => println!("Invalid option. Please try again."),
                }
            }
        }
    }
}