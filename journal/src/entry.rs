// Imports for reading and writing files
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
// Struct hold attributes of date and log
pub struct Entry {
    pub date: String,
    pub log: String,
}

// Methods
impl Entry {
    // Constructor
    pub fn new() -> Entry {
        Entry {
            date: String::new(),
            log: String::new(),
        }
    }
}