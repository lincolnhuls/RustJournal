# Rust Journal and Calculator

I wrote two projects to accomplish the goals of this module. The first program is a basic caluclator which made use of multiple functions
to accomplish the basic operations of addition, subtraction, multiplication, and division. The second project is a journal application that allows for a user to save and load a journal. Once inside of the journal, they are able to create and view subject, and select a subject. Once in a subject, they are able to create and view entries.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Pull the latest version from GitHub
2. Cd into the project directory
3. Run cargo run to start the program

Instructions for using the software:

1. Once the program is running, create a new journal or load an existing one
2. Pick an option from the main menu
3. Once a subject is selected, pick an option from the subject menu
4. When read, save the journal and exit the program

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

- Dowload Rust and dependencies from [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- The only needed libraries are already included in the cargo.toml file and should compile automatically

## Useful Websites to Learn More

I found these websites useful in developing this software:

- [Create serde_json documentation](http://docs.rs/serde_json/latest/serde_json/)
- [Rust Tutorial (W3Schools)](https://www.w3schools.com/rust/)
- [Rust Documentation](https://doc.rust-lang.org/stable/)
- [Rust Programming Full Course (YouTube)](https://www.youtube.com/watch?v=rQ_J9WH6CGk&t=25s)
- [Rust in 100 Seconds (YouTube)](https://www.youtube.com/watch?v=5C_HPTJg5ek)
- [Rust for the impatient (YouTube)](https://www.youtube.com/watch?v=br3GIIQeefY&t=83s)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

- I want to clean up the terminal any time anyone backtracks somewhere, right now if you stay in a menu it can get cluttered.
- I want to add an ability to create bullet lists
- I want to add a way to edit previous entries as well as delete them
- To improve it I want subjects to be capitalized automatically instead of lowercase
