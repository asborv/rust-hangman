use std::io;

fn main() {
    let secret = "hei";

    loop {

        let mut guess = String::new();
        println!("Input guess >>>");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: char = match guess.trim().chars().nth(0) {
            Some(c) => { c }
            None => {
                println!("Uh Oh");
                continue;
            }
        };
        println!("Your guess is {:?}", guess);

        let i = match secret.find(guess) {
            Some(n) => {
                println!("Correct at index {n}");
                n
            },
            None => {
                println!("Incorrect.");
                continue;
            }
        };
        println!("{i}");
        



        // Create secret word
        //
        // print secret word with revealed chars
        // 
        // read user guess
        //
        // check if guess in secret
        //
        // if yes: uncover all chars where guess in secret
        // if no: decrement lives
    }
}
