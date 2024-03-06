mod words;
use std::io::Write;
use colored::Colorize;
use std::io;
use std::process;


fn get_guess() -> String {
    let mut guess = String::new();

    // prompt the user to enter a guess
    print!("Please input your guess: "); 

    // read the guess
    std::io::stdout().flush().unwrap(); 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //print!("{guess}"); // print check
    // check/return the guess
    guess = guess.trim().to_string();
    if words::is_word_valid(&guess.trim()) {
        return guess.to_uppercase();
    }
    else{
        println!("Your guess was not valid. Try again.");
        return get_guess();
    }
}

/// Colors `ch` black on green.
fn to_green(ch: char) -> String {
    ch.to_string()
      .black()
      .on_green()
      .to_string()
}

/// Colors `ch` black on yellow.
fn to_yellow(ch: char) -> String {
    ch.to_string()
      .black()
      .on_yellow()
      .to_string()
}

/// Colors `ch` black on white.
fn to_gray(ch: char) -> String {
    ch.to_string()
      .black()
      .on_white()
      .to_string()
}

fn color_guess(guess: &str, secret: &str) -> String {
    let mut secret_chars: Vec<char> = secret.chars().collect();
    let mut colored_guess = String::new();

    for ch in guess.chars() {
        if let Some(pos) = secret_chars.iter().position(|&c| c == ch) {
            secret_chars.remove(pos);
            if pos == 0 {
                colored_guess.push_str(&to_green(ch));
            } else {
                colored_guess.push_str(&to_yellow(ch));
            }
        } else {
            colored_guess.push_str(&to_gray(ch));
        }
    }
    colored_guess
}

fn main() {
    let mut guess_count = 0;
    let secret = words::random_word();
    let mut guess_history = String::new();

    while guess_count < 6 {
        let guess_input = get_guess();
        let mut guess_colored = String::new();
        guess_colored.push_str(&color_guess(&guess_input, &secret));
        println!("{}", guess_history);
        println!("{}", guess_colored);
        guess_history.push_str("\n");
        guess_history.push_str(&guess_colored);

        // Check if guess is correct
        if guess_input.trim().to_uppercase() == secret {
            println!("Congratulations! You guessed the word correctly!");
            process::exit(0); // Exit the program with exit code 0 (success)
        }

        guess_count += 1; // Increment guess count after each guess
    }
    print!("You've exceeded the guess count. hold dis L");
}