use guess_word::*;
use std::io;

fn main() {
    let game = Game::default();
    let mut guess = String::new();
    let answer = game.get_answer();
    println!("{}", answer);

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let trimmed_guess = guess.trim();
        if game.in_dictionary(&trimmed_guess) {
            if answer == guess.trim() {
                println!("You Win!");
                break;
            } else {
                println!("Not match word...");
            }
        } else {
            println!("Well... Whait's?");
        }

        guess.clear();
    }
}
