use std::{fmt, io};
use rand::Rng;

fn main() {
    println!("Thanks for playing Rock Paper Scissors!");

    let mut game_stats = GameStats::new();

    let mut keep_playing = true;
    while keep_playing == true {
        let mut rounds_won = 0;
        let mut rounds_lost = 0;
        let mut rounds_tied = 0;

        while rounds_won + rounds_lost + rounds_tied < 3 {
            let player_choice = get_user_choice();
            game_stats.update_player_choice_stats(&player_choice);

            let ai_choice = get_ai_choice();

            let round_summary = format!("You played {} and the computer played {}", &player_choice, &ai_choice);
            let round_result = get_game_result(player_choice, ai_choice);

            // update round stats and provide message
            match round_result {
                GameResult::Win => {
                    rounds_won += 1;
                    println!("{} - You won!", round_summary);
                },
                GameResult::Loss => {
                    rounds_lost += 1;
                    println!("{} - You lost!", round_summary);
                },
                GameResult::Tie => {
                    rounds_tied += 1;
                    println!("{} - You tied!", round_summary);
                }
            }
        }

        // calculate win / loss across rounds
        if rounds_won > rounds_lost {
            println!("CONGRATS! You won the game!");
            game_stats.update_game_results(&GameResult::Win);
        } else if rounds_lost > rounds_won {
            println!("Oh no! You lost the game!");
            game_stats.update_game_results(&GameResult::Loss);
        } else {
            println!("The game ended in a tie!");
            game_stats.update_game_results(&GameResult::Tie);
        }

        // print current game stats
        game_stats.print_game_stats();

        // ask player if they want to keep playing

        println!("Would you like to keep playing?");
        println!("press q to quit or any key to keep going");

        let mut press_continue = String::new();
        io::stdin().read_line(&mut press_continue).expect("failed to read user input");
        if press_continue.trim() == "q" {
            println!("Thanks for playing!");
            keep_playing = false;
        }

    }
}

// declare game enums
pub enum RspChoice {
    Rock,
    Paper,
    Scissors
}
impl fmt::Display for RspChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RspChoice::Rock => write!(f, "Rock"),
            RspChoice::Paper => write!(f, "Paper"),
            RspChoice::Scissors => write!(f, "Scissors"),
        }
    }
}

pub enum GameResult {
    Win,
    Loss,
    Tie
}

// stub out major functions
fn get_user_choice() -> RspChoice {
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
fn get_ai_choice() -> RspChoice {
    let num = rand::thread_rng().gen_range(0..2);
    match num {
        0 => RspChoice::Rock,
        1 => RspChoice::Paper,
        2 => RspChoice::Scissors,
        _ => panic!("Error: out-of-range value for random number between 0 - 2")
    }
}

// compare user and ai choices and declare win / loss
fn get_game_result(user_choice: RspChoice, ai_choice: RspChoice) -> GameResult {
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

// store info on game stats
pub struct GameStats {
    wins: i8,
    losses: i8,
    ties: i8,
    rock_picks: i16,
    paper_picks: i16,
    scissor_picks: i16,
}

impl GameStats {
    pub fn new() -> Self {
        Self {
            wins: 0,
            losses: 0,
            ties: 0,
            rock_picks: 0,
            paper_picks: 0,
            scissor_picks: 0
        }
    }

    pub fn update_game_results(&mut self, result: &GameResult) {
        match result {
            GameResult::Win => self.wins += 1,
            GameResult::Loss => self.losses += 1,
            GameResult::Tie => self.ties += 1,
        }
    }

    pub fn get_win_ratio(&self) -> i8 {
        let total_games = self.wins + self.losses + self.ties;
        if self.wins == 0 {
            0
        } else {
            let ratio = (self.wins * 100) / total_games;
            ratio
        }
    }

    pub fn update_player_choice_stats(&mut self, choice: &RspChoice) {
        match choice {
            RspChoice::Rock => self.rock_picks += 1,
            RspChoice::Paper => self.paper_picks += 1,
            RspChoice::Scissors => self.scissor_picks += 1,
        }
    }

    pub fn print_game_stats(&self) {
        println!("Wins: {}", self.wins);
        println!("Losses: {}", self.losses);
        println!("Ties: {}", self.ties);
        println!("Win Ratio: {}%", self.get_win_ratio());
        println!("Rock Picked: {} times", self.rock_picks);
        println!("Paper Picked: {} times", self.paper_picks);
        println!("Scissors Picked: {} times", self.scissor_picks);
    }
}