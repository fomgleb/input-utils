use std::{
    io::{self, Write},
    str::FromStr,
};

/// Reads input from the standard input stream and attempts to parse it into the specified type.
/// Continuously prompts for input until a valid input is entered.
///
/// # Arguments
///
/// * `prompt` - A string slice that holds the text to be displayed as a prompt before reading input.
/// * `err` - A string slice that holds the text to be displayed as an error message if parsing fails.
///
/// # Returns
///
/// Returns the parsed value of type `T` if parsing is successful. If parsing fails, the function will
/// continue to prompt for input and display the error message until a valid input is entered.
///
/// # Example
///
/// ```
/// let age: u32 = read_input("Enter your age: ", "Invalid input. Please enter a valid age.");
/// println!("Your age is {}", age);
/// ```
///
/// # Panics
///
/// The function will panic if flushing stdout or reading from stdin fails.
pub fn read_input<T: FromStr>(prompt: &str, err: &str) -> T {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        print!("\n");
        match input.trim().parse() {
            Ok(parsed_input) => break parsed_input,
            Err(_) => eprintln!("{}", err),
        }
    }
}
