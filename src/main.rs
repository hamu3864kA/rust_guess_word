use guess_word::*;
use std::io;
use iced::{Application, Settings};
mod gui; // `gui.rs` の gui モジュールを参照する

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 600);
    gui::GuessWord::run(settings)
}

fn _main() {
    let mut game = Game::default();
    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let (state, result) = game.guess(guess.trim());

        match state {
            GameStatus::Won => {
                println!("You Win!");
                break;
            },
            GameStatus::Lost => {
                println!("You Lost...(answer: {})", game.get_answer().unwrap());
                break;
            },
            GameStatus::InProgress => match result {
                GuessResult::DuplicateGuess => println!("Warning: Duplicate Guess."),
                GuessResult::IncorrectLength => println!("Warning: Incorrect Length."),
                GuessResult::NotInDictionary => println!("Warning: Not in dictionary."),
                GuessResult::Valid => {
                    let word_guess = game.guesses().last().unwrap();
                    let result: String = word_guess
                        .letters()
                        .iter()
                        .map(|l| match l.accuracy {
                            HitAccuracy::InRightPlace => "*".to_string(),
                            HitAccuracy::InWord => "!".to_string(),
                            HitAccuracy::NotInWord => " ".to_string(),
                        })
                        .collect();
                        println!("{}", result);
                },
                GuessResult::GameOver => println!("Warning: Game Over."),
            },
        }
        guess.clear();
    }
}
