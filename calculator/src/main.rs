use std::{io};
use std::io::Write;

fn addition(){
    let mut value = String::new();
    let mut again = true;
    let mut total = 0.0;
    while again {
        value.clear();
        print!("Please enter a value: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).expect("Please enter a value.");
        let trim_value = value.trim().to_lowercase();
        if trim_value == "quit" {
            again = false;
        } 
        else if let Ok(num_value) = trim_value.parse::<f64>() {
            total = total + num_value;
        } 
        else {
            println!("Please enter a valid choice.");
        }
    }
    println!("The final value is: {}", total);
}

fn subtraction(){
    let mut inital_value = String::new();
    let mut value = String::new();
    let mut again = true;
    let mut total = 0.0;
    loop {
        inital_value.clear();
        print!("Enter a number to start suntracting from: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inital_value).expect("Please enter a value.");
        let trim_inital_value = inital_value.trim().to_lowercase();
        if let Ok(num_inital_value) = trim_inital_value.parse::<f64>() {
            total = total + num_inital_value;
            break;
        }
        else {
            println!("Please enter a number.");
        }
    }
    while again {
        value.clear();
        print!("Please enter a value: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).expect("Please enter a value.");
        let trim_value = value.trim().to_lowercase();
        if trim_value == "quit" {
            again = false;
        }
        else if let Ok(num_value) = trim_value.parse::<f64>() {
            total = total - num_value;
        }
        else {
            println!("Please enter a valid choice");
        }
    }
    println!("The final value is: {}", total);
}

fn multiplication(){
    let mut value = String::new();
    let mut again = true;
    let mut total = 1.0;
    while again {
        value.clear();
        print!("Please enter a value: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).expect("Please enter a value.");
        let trim_value = value.trim().to_lowercase();
        if trim_value == "quit" {
            again = false;
        } 
        else if let Ok(num_value) = trim_value.parse::<f64>() {
            total = total * num_value;
        } 
        else {
            println!("Please enter a valid choice.");
        }
    }
    println!("The final value is: {}", total);
}

fn division(){
    let mut inital_value = String::new();
    let mut value = String::new();
    let mut again = true;
    let mut total = 0.0;
    loop {
        inital_value.clear();
        print!("Enter a value to start dividing from: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inital_value).expect("Please enter a value.");
        let trim_inital_value = inital_value.trim().to_lowercase();
        if let Ok(num_inital_value) = trim_inital_value.parse::<f64>() {
            total = total + num_inital_value;
            break;
        }
        else {
            println!("Please enter a number.");
        }
    }
    while again {
        value.clear();
        print!("Please enter a value: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).expect("Please enter a value.");
        let trim_value = value.trim().to_lowercase();
        if trim_value == "quit" {
            again = false;
        }
        else if let Ok(num_value) = trim_value.parse::<f64>() {
            total = total / num_value;
        }
        else {
            println!("Please enter a valid choice");
        }
    }
    println!("The final value is: {}", total);
}

fn calculator() {
    let mut again = true;
    let mut input = String::new();
    while again {
        input.clear();
        println!("Please select an operation[Add, Sub, Mult, Div, Quit]: ");
        io::stdin().read_line(&mut input).expect("Please enter a valid choice");
        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "add" => {
                println!("Addition selected. Please enter number to add, type quit to get result.");
                addition();
            },
            "sub" => {
                println!("Subtraction selected. Please enter number to subtract, type quit to get result.");
                subtraction();
            },
            "mult" => {
                println!("Multiplication selected. Please enter number to multiply, type quit to get result.");
                multiplication();
            },
            "div" => {
                println!("Divison selected. Please enter number to divide, type quit to get result.");
                division();
            },
            "quit" => again = false,
            _ => {
                println!("Invalid choice.");
                calculator();
            }
        }
    }
}

fn main() {
    calculator();
}
