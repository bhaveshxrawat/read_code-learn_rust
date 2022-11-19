use std::io;

fn main() {
    println!("Let's play, 'Guess the Number'"); //println -> prints the text in arguments along with adding a new line.
    println!("Enter you number");

    let mut usernumber = String::new(); //mut -> By default, variables in Rust are iummutable, adding mut prior to variable name makes the variable mutable.
    io::stdin() // this line basically uses stdin function from the 'io' library/module that we've imported from the std library.
    .read_line(&mut usernumber) // read_line basically reads the the user typed content to the variable it has as an argument.
    // .read_line method is called on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. 
    // The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument.
    // The string argument needs to be mutable so the method can change the string’s content.
    // Also, the '&' is for reference of the variable, it basically lets the rustacean tells the address of the variable rather than needing to copy the vairable itself. This improves memory efficiency.
    .expect("You guessed wrong"); // This line is basically kind of like a fallback code. When a coder codes, it expects an error, here we can enter the text we want to see on the error as arguments.
    // .read_line returns a result that has different states also known as variants. 
    // Result's variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.
    println!("You guessed {usernumber}");
}
