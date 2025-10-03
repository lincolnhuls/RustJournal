#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]

/*
This will work as my living document as I learn rust, all things I will learn will be placed here.
Create a new rust project without github being auto setup - cargo new project_name --vcs none
One of the main things that makes rust standout is it is "safe by default"
Variables only live in the block they were created in, {}
Rust uses variable shadowing. If a variable is declared with the same name as another in the 
    same scope, it covers the previous variable and value, making them inaccessable(use let keyword in declaration)

*/

// Import hashmap
use std::collections::HashMap;

// Creating a function, use the fn keyword, the function name, () and {}
fn my_function() {
    println!("Inside my funciton");
}

// For parameters declare the name: type, return type is listed after parameters
fn add_numbers(number1: i32, number2: i32) -> i32 {
    let total = number1 + number2;
    return total;
}

fn main() {
    println!("Hello, world!");

    // To create an immutable(changeable) variable, use the let keyword
    let name = "Lincoln";
    let class = "CSE310";
    // For formatted string use {}, followed by the variables in the ordered desired after
    println!("My name is {} and I am learning rust for {}!", name, class);

    // Add the mut keyword(mutable) to allow for variables to change after declaration
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);

    // Rust automatically delcares variable type, you do not have to declare it like in c++
    let my_num = 5;
    let my_double = 5.99;
    let my_letter = "D";
    let my_bool = true;
    let my_text = "Hello";

    // You can declare if you would like
    let my_num: i32 = 5;
    /* Common data types - 
    Numbers - Whole numbers and decimal numbers (i32, f64)
    Characters - Single letters or symbols (char)
    Strings - Text, a sequence of characters (&str)
    Booleans - True or false values (bool)
    */

    // To create const variables, you must declare a type (UML is to write these in all CAPS)
    const BIRTHYEAR: i32 = 1980;

    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    let mut x = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);

    // Logic operations
    // && - And
    // || - Or
    // ! - Not

    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    // Rust can identify boolean from comparisons
    let age = 20;
    let can_vote = age >= 18;

    println!("Can you vote? {}", can_vote);

    // If statement format, uses else if 

    let this = true;

    if this {
        println!("True");
    } else {
        println!("False");
    }

    // Can use it in an epression
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);

    // Match is more effective than writing lots of long if..else chains

    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    // The match is prefomed only once against all "branches" of the variable day
    // _ is used to specify what code to run in the event no match is found

    // Match comparison can be coupled with the | (or) operatior
    let day = 6;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid Day"),
    }

    // Like if statements, match can return a result

    let day = 1;

    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day",
    };

    println!("{}", result);

    // Rust has the normal for and while loops, but also uses the loop loop, which runs infinitly unless you end it, done using the break keyword
    let mut count = 1;

    let result = loop{
        println!("In loop {}", count);
        if count == 3 {
            println!("Loop broken");
            break count;
        }

        count += 1;
    };
    println!("The final count was {}", result);
    
    // While loop syntax
    let mut count = 1;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // The while loop checks conditions before each loop, so if initial value is false, the loop will not run
    let count = 10;

    while count <= 5 {
        println!("This will not print");
    }

    //  You can exit a while loop early by using break and skip a value using continue
    let mut count = 0;

    while count <= 10 {
        if count == 6 {
            count += 1;
            continue
        }
        println!("Count: {}", count);
        count += 1;
    }

    // For loop syntax, 1..6 is one up to but not inlcuding 6, so only 5 values will be printed
    for i in 1..6 {
        println!("i is: {}", i);
    }

    // To include last number use inclusive range ..=
    for i in 1..=6 {
        println!("i is: {}", i);
    }

    // Can also use break and continue, this example skips 3 and stops at 5, but doesn't print 5 because the loop quits before then
    for i in 1..=10 {
        if i == 3 {
            continue;
        }
        if i == 5 {
            break;
        }
        println!("i is: {}", i);
    }
    
    // Call functions like normal
    my_function();

    // List creation in rust is a bit different
    // First is a vector, with explicit types and sizes
    let numbers: [i32; 5] = [1,2,3,4,5];

    // Some types can be implied
    let fruits = ["apple", "banana", "orange"];

    // Array with repeated values, can handel the creation for you
    let zeros = [0; 10]; // Create [0,0,0,0,0,0,0,0,0,0]

    // The most common type and the closest to a typical "list" is a vector, can have varaible lenght, arrays cannot
    let mut numbers: Vec<i32> = Vec::new();

    // Can be created using the vec! macro
    let mut fruits = vec!["apple", "banana", "orange"];

    fruits.push("grape");

    println!("First fruit: {}", fruits[0]);

    // Vector with type annotation
    let mut scores: Vec<i32> = vec![10,20,30];

    // Slices 
    let numbers = [1,2,3,4,5];
    let slice = &numbers[1..4]; // References elements 1,2,3

    // Create a mutable vector
    let mut my_list = vec![1, 2, 3, 4, 5];
    
    // Add elements
    my_list.push(6);
    
    // Remove last element
    if let Some(last) = my_list.pop() {
        println!("Removed: {}", last);
    }
    
    // Iterate through the list
    for item in &my_list {
        println!("Item: {}", item);
    }
    
    // Access by index
    println!("First item: {}", my_list[0]);
    
    // Get length
    println!("Length: {}", my_list.len());

    my_function();
    let sum = add_numbers(2, 5);
    println!("Sum: {}", sum);

    // Shadowing
    let x = 5;
    let x = 10;
    println!("Value: {}", x);

    // Scope
    let x = 5;

    {
        let x = 10;
        println!("Inside block: {}", x);
    }

    println!("Outside block: {}", x);

    // Create strings using 
    let greeting: &str = "Hello!";
    println!("{}", greeting);

    // &str is for string slices, for fixed text
    // String is for strings that can change

    // Create strings from literals
    let text1 = "Hello World".to_string();
    let text2 = String::from("Hello World");

    // Strings must have mut to change and are changed with push_str to add to
    let mut greeting = String::from("Hello");
    greeting.push_str(" World");
    println!("New Greeting: {}", greeting);

    // Use push to add one charater, charaters use '' not ""
    greeting.push('!');
    println!("Added charater: {}", greeting);

    // Combine strings using format!
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = String::from(" to you");
    let result = format!("{}{}{}", s1, s2, s3);
    println!("{}", result);

    // .len() for the length of a string
    let name = String::from("Lincoln");
    println!("Length: {}", name.len());

    // Ownership
    let a = String::from("Hello");
    // Trasfer ownership from a to b
    let b = a;
    
    // println!("{}, a); Will give an error because a no longer has ownership
    println!("{}", b);

    // Simple types (numbers, charaters, and booleans), are copied, not moved so you can still use
    // them after a reassignment
    let a = 5;
    let b = a;
    println!("{}", a);
    println!("{}", b);    

    // If you want to keep the original value in the variable, you can clone it
    let a = String::from("Hello");
    let b = a.clone();
    println!("{}", a);
    println!("{}", b);

    // If you want to use a value without changing ownership you can borrow it
    let a = String::from("Hello");
    let b = &a;
    println!("a = {}", a);
    println!("b = {}", b);

    // To change a value through a refrence it needs to be mutable
    let mut name = String::from("Lincoln");
    let name_ref = &mut name;
    name_ref.push_str(" Huls");
    println!("{}", name_ref);

    // Tuples
    let person = ("Lincoln", 22, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is Active: {}", person.2);

    // Use hashmaps to store key value pairs, allows look ups using keys
    // Have to import it 
    let mut capitalCities = HashMap::new();
    capitalCities.insert("France", "Paris");
    capitalCities.insert("Japan", "Tokyo");
    println!("Captial of Japan is {}", capitalCities["Japan"]);

    // Change array values 
    let mut numbers = [1, 2, 3, 4, 5];
    numbers[0] = 10;
    println!("The new first number is: {}", numbers[0]);

    // Can also get length using len()
    let numbers = [1, 2, 3, 4, 5];
    println!("This array has {} elements.", numbers.len());

    // Loop through an array
    let fruits = ["apple", "banana", "orange"];
    for fruit in fruits {
        println!("I like {}.", fruit);
    }

    // To print a full array use {:?}
    let number = [1,2,3,4,5];
    println!("{:?}", numbers);

    // Delcare a vector with the vec! macro
    let fruits = vec![1,2,3,4];

    // Add values to a vector 
    let mut fruits = vec!["apple", "banana"];
    fruits.push("cherry");
    println!("{:?}", fruits); // ["apple", "banana", "cherry"]

    // Remove items from a vector 
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.pop();
    println!("{:?}", fruits); // ["apple", "banana"]

    // Add to a specific spot
    let mut fruits = vec!["banana", "orange"];
    fruits.insert(1, "apple");
    println!("{:?}", fruits); // ["banana", "apple", "orange"]

    // Remove items from vector at specific index
    let mut fruits = vec!["apple", "banana", "orange"];
    fruits.remove(0);
    println!("{:?}", fruits); // ["banana", "orange"]

    // Get length of a vector
    let fruits = vec!["apple", "banana", "cherry"];
    println!("There are {} fruits.", fruits.len());

    // Loop through a vector
    let fruits = vec!["apple", "banana", "orange"];
    for fruit in &fruits {
        println!("I like {}.", fruit);
    }

    // Create a tuple
    let person = ("Lincoln", 30, true);

    // Unpack a tuple
    let person = ("Jenny", 45, false);
    let (name, age, active) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active);

    // Get values from a HashMap
    let mut capitalCities = HashMap::new();

    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    if let Some(city) = capitalCities.get("England") {
        println!("The capital of England is {}.", city);
    } else {
        println!("England is not in the map.");
    }

    // Update values in a HashMap
    let mut capitalCities = HashMap::new();

    capitalCities.insert("England", "London");
    capitalCities.insert("England", "Berlin");

    println!("{:?}", capitalCities);

    // Remove HashMap values
    let mut capitalCities = HashMap::new();

    // Add keys and values (Country, City)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    // Remove the key "England"
    capitalCities.remove("England");

    println!("{:?}", capitalCities);

    // Loop through a HashMap
    let mut capitalCities = HashMap::new();

    // Add keys and values (Country, City)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    // Loop through the HashMap
    for (country, city) in &capitalCities {
        println!("The capital of {} is {}.", country, city);
    }

    // Struct are pretty much little databases for a single thing
    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }

    // Once created, you can create an object of it
    let user = Person {
        name: String::from("Lincoln"),
        age: 22,
        can_vote: true,
    };

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Can vote? {}", user.can_vote);

    // To change values in a struct the mut keyword must be used
    struct NewPerson {
        name: String,
        age: u32,
    }

    let mut user = NewPerson {
        name: String::from("John"),
        age: 35,
    };

    user.age = 36; // Change value of age
    println!("Name: {}", user.name);
    println!("Updated age: {}", user.age);

    // Enums are good for representing values that can only be a set of options, like days in a week
    enum Weekdays {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // To use enum create a variable and assign it a value using ::
    let my_day = Weekdays::Monday;

    // Work well with match statements
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let my_direction = Direction::Left;

    match my_direction {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
    }

    // Enums can hold data
    enum LoginStatus {
        Success(String),
        Error(String),
    }

    let result1 = LoginStatus::Success(String::from("Welcome Lincoln!"));
    let result2 = LoginStatus::Error(String::from("Incorrect Password"));

    match result1 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        LoginStatus::Error(message) => println!("Error: {}", message),
    }
}
