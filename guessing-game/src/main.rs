use std::io;  // "std" is a standard library. "io" is an input/output library in the standard library. 

use rand::Rng; 
// main function is the entry point of the program 
fn main() {  
    // println! is a macro that prints the string to the screen 
    println!("Guess the number!"); 

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println! ("The secret number is {secret_number}");

    println!("Please input your guess."); 
    // mut for mutable variable, string::new() creates a new empty string, :: is a path seperator, operator to navigate name spaces 
    // new is an associated function of the String type, an associated function is a function implemented on a type in this case string 
    // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
    let mut guess = String::new();  // created a mutable variable that is currently bound to a new empty instance of string.

    io::stdin() // call the stdin function allowing us to handle user input 
    .read_line(&mut guess) //read_line is a method on the standard input handle to get input from the user 
    // &mut guess is an argument to read_line 
    // & indicates that this argument is a reference which gives us a way to let multiple parts of the code access one piece of data without needing to copy that data into memory multiple times 
    
    // referencing is like borrowing with lifetimes and doesn't trigger re-render 

    // 
    .expect("Failed to read line");   // expect is like an if else throw error pattern in one line. 

    println!("You guessed {guess}")
}