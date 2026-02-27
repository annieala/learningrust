use std::io;
use std::cmp::Ordering; // ordering type is another enum and has the variants Less, Greater and Equal. the cmp method compares two values and can be called on anything that can be compared. it takes reference to whatever you want to compare with.  

use rand::Rng;
// Rng is a trait that defines methods that random number generators implement. we bring the Rng trait into scope to use the gen_range method on the random number generator we create in the next line.
// main function is the entry point of the program 
fn main() {  
    // println! is a macro that prints the string to the screen 
    println!("Guess the number!"); 

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println! ("The secret number is {secret_number}");

    loop { 
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

    let guess: u32 = match guess.trim().parse() { 
        Ok(num) => num, // if the parse is successful, we get the number back and bind it to the variable guess 
        Err(_) => continue, // if the parse fails, we get an error back and we ignore the error and start a new iteration of the loop
    };

    // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadow the previous value of guess with a new one, lets us reuse the guess variable rather than forcing us to create two unique varaibles such as guess_str and guess. 
    // trim eliminates any whitespace at the beginning and end of the string, including the newline character that is added when the user presses enter.
    // parse converst a string to another type. 
    // u32 unsigned 32 bit integer. 
    println!("You guessed {guess}");

    match guess.cmp(&secret_number) { // cmp is a method that compares the guess to the secret number and returns a variant of the Ordering enum we brough into scope with the use statement. the match expression decides what to do next based on which variant of ordering was returned from the call to cmp with the values in guess and secret_number
        // a match expression is made up of arms an arm consists of a pattern to match against and the code that should be run if the value given to match fitms that arms pattern. rust takes the value given to match and looks through each arms parrtern in turn. 
        Ordering::Less => println!("Too small!"), // if the guess is less than the secret number, print "Too small!"
        Ordering::Greater => println!("Too big!"), // if the guess is greater than the secret number, print "Too big!"
        Ordering::Equal => {
            println!("You win!"); // if the guess is equal to the secret number, print "You win!"
            break;
        }
    }
    }

}
