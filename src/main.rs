// Prelude
// Imports the io library from the C standard library
use std::io;

fn main() {
    println!("Guess the Number!");

    println!("Please input your guess.");
    
    // Variable declaration
    // Variables are immutable  by default
    // mut allows them to be mutable
    // ::new() means that new is an associated function of the string type
    // result: created a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess = String::new();

    //  The stdin function returns an instance of std::io::Stdin
    //  std::io::Stdin is a type that represents a handle to the standard input 
    io::stdin()
    // & denotes a reference parameter
    // read_line takes in user input and returns a value, in this case an io::Result
    // Rust has multiple types named result
    //  - Geeneric Result
    //  - Specific versions for submodules, like, io::Result
    //  - Result types are enumerations(type with a fixed set of values called variants)
    //  - The variants here are ok and Err
        .read_line(&mut guess)
    // Expect will take an Ok value and hold it for use
    //  - Will crash for an err value  and display the message we passed
        .expect("Failed to read line");
    // This isn't the best error handling. That will be covered later.

    
    println!("You guessed: {}", guess);
}
