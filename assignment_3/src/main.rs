// Function to check the guess
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0  // Correct guess
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Hard-code the secret number
    let secret_number: i32 = 42;

    // Variable to count the number of guesses
    let mut guess_count = 0;

    // Start the guessing loop
    loop {
        // Simulate a guess (hard-coded for now)
        let guess: i32 = 30 + guess_count;

        // Increment the guess count
        guess_count += 1;

        // Check the guess
        let result = check_guess(guess, secret_number);

        // Print the result
        if result == 0 {
            println!("The guess {} is correct!", guess);
            break; // Exit the loop if the guess is correct
        } else if result == 1 {
            println!("The guess {} is too high.", guess);
        } else {
            println!("The guess {} is too low.", guess);
        }
    }
    
    println!("It took {} guesses to find the secret number.", guess_count);
}
