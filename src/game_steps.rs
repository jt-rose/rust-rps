use std::io;
use rand::Rng;
use crate::game_result::GameResult;
use crate::rsp_choice::RspChoice;

pub fn get_user_choice() -> RspChoice {
    println!("Select: r - rock, p - paper, s - scissors");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("failed to read user input");
    match choice.trim() {
        "r" => RspChoice::Rock,
        "s" => RspChoice::Scissors,
        "p" => RspChoice::Paper,
        _ => get_user_choice()
    }
}

// randomly generate an RSP choice
pub fn get_ai_choice() -> RspChoice {
    let num = rand::thread_rng().gen_range(0..2);
    match num {
        0 => RspChoice::Rock,
        1 => RspChoice::Paper,
        2 => RspChoice::Scissors,
        _ => panic!("Error: out-of-range value for random number between 0 - 2")
    }
}

// compare user and ai choices and declare win / loss
pub fn get_game_result(user_choice: RspChoice, ai_choice: RspChoice) -> GameResult {
    match user_choice {
        RspChoice::Rock => {
            match ai_choice {
                RspChoice::Rock => GameResult::Tie,
                RspChoice::Paper => GameResult::Loss,
                RspChoice::Scissors => GameResult::Win,
            }
        },
        RspChoice::Paper => {
            match ai_choice {
                RspChoice::Rock => GameResult::Win,
                RspChoice::Paper => GameResult::Tie,
                RspChoice::Scissors => GameResult::Loss,
            }
        },
        RspChoice::Scissors => {
            match ai_choice {
                RspChoice::Rock => GameResult::Loss,
                RspChoice::Paper => GameResult::Win,
                RspChoice::Scissors => GameResult::Tie,
            }
        }
    }
}

