// Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;
// Imports the io library from the C standard library because std::io is not included in the prelude
use std::io;
use std::cmp::Ordering;

// INTERESTING NOTE:
// Instructions for using a crate are in each crate’s documentation. 
// A feature of Cargo is that you can run the cargo doc --open command, 
// which will build documentation provided by all of your dependencies locally and open it in your browser.

fn main() {
    println!("Guess the Number!");

    // rand::thread_rng() will give us the random number generator that is local to the current thread of execution and seeded by the operating system
    // then we call the gen_ranger
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    // Variable declaration
    // Variables are immutable  by default
    // mut allows them to be mutable
    // ::new() means that new is an associated function of the string type
    // result: created a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess = String::new(); //String::new() means that new is an associated function(a function implemented by a type)

    //  The stdin function returns an instance of std::io::Stdin
    //  std::io::Stdin is a type that represents a handle to the standard input for the terminal
    io::stdin()
        // & denotes a reference parameter
        // read_line takes in user input and returns a value, in this case an io::Result
        // Rust has multiple types named result
        //  - Generic Result
        //  - Specific versions for submodules, like, io::Result
        //  - Result types are enumerations(type with a fixed set of values called variants)
        //  - The variants here are ok and Err
        .read_line(&mut guess)
        // Expect is a method built on to the io::Result oject
        // Expect will take an Ok value and hold it for use
        //  - Will crash for an err value  and display the message we passed
        .expect("Failed to read line");
    // This isn't the best error handling. That will be covered later.

    println!("You guessed: {}", guess);

    // NOTE: This does not work. I just couldn't finish the chapter yet
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You Win!")
    }
}
