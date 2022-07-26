use std::io;

fn main() {
    // Initialize game variables
    let mut lives = 5;          // No. of allowed errors
    let secret = "heei";         // Word to guess
    let mut mask = Vec::new();  // Maps correct guesses to secret

    // Start with no correct guesses
    for _char in secret.chars() {
        mask.push(false);
    }

    // Game loop
    loop {

        let mut guess = String::new();
        let mut out = String::new();

        // Create string to represent correct guesses
        mask.iter().enumerate().for_each(|(i, show)| {
            out.push(
                if *show {
                    match secret.chars().nth(i) {
                        Some(c) => { c },
                        None => {
                            println!("Uh oh!");
                            '_'
                        }
                    }
                } else {
                    '_'
                }
            );
        });

        // Show info and prompt
        println!("You have {} lives", lives);
        println!("The word: {}", out);
        println!("Input guess >>>");

        // Read and parse guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: char = match guess.trim().chars().nth(0) {
            Some(c) => { c }
            None => {
                println!("Failed to parse guess. Input a letter.");
                continue;
                }
        };

        // Find all guess occurrences in secret
        let correct_indices: Vec<_> = secret.match_indices(guess)
            .map(|(i, _)| i)
            .collect();

        // Decrement lives if incorrecct and prompt user
        if correct_indices.len() == 0 {
            println!("{} is incorrect.", guess);
            lives -= 1;
        } else {
            println!("{} is collect.", guess);
        }

        // Update mask accoring to correct indices
        correct_indices.iter()
            .for_each(|i| mask[*i] = true);

        // Win once all letters are correctly guessed
        if mask.iter().all(|x| *x) {
            println!("You win!");
            break;
        }

        // Lose when player runs out of lives
        if lives == 0 {
            println!("You lose!");
            break;
        }
    }
}
