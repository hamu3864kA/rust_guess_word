use guess_word::*;
use std::io;

fn main() {
    let mut game = Game::default();
    let mut guess = String::new();

    println!("{:?}", game.get_test_anser());

    // if let Ok(x) = game.get_answer() {
    //     println!("({})", x);
    // };

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
