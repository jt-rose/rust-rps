use std::io;
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

            let round_summary = "You played " + &player_choice + " and the computer played " + &ai_choice;
            let round_result = get_game_result(player_choice, ai_choice);

            // update round stats and provide message
            match round_result {
                GAME_RESULT::Win => {
                    rounds_won += 1;
                    println!("{} - You won!", round_summary);
                },
                GAME_RESULT::Loss => {
                    rounds_lost += 1;
                    println!("{} - You lost!", round_summary);
                },
                GAME_RESULT::Tie => {
                    rounds_tied += 1;
                    println!("{} - You tied!", round_summary);
                }
            }
        }

        // calculate win / loss across rounds
        if rounds_won > rounds_lost {
            println!("CONGRATS! You won the game!");
            game_stats.update_game_results(&GAME_RESULT::Win);
        } else if rounds_lost > rounds_won {
            println!("Oh no! You lost the game!");
            game_stats.update_game_results(&GAME_RESULT::Loss);
        } else {
            println!("The game ended in a tie!");
            game_stats.update_game_results(&GAME_RESULT::Tie);
        }

        // print current game stats

        // ask player if they want to keep playing

        println!("Would you like to keep playing - y / n ?")

    }
}

// declare game enums
enum RSP_CHOICE {
    Rock,
    Paper,
    Scissors
}

enum GAME_RESULT {
    Win,
    Loss,
    Tie
}

// stub out major functions
fn get_user_choice() -> RSP_CHOICE {
    println!("Select: r - rock, p - paper, s - scissors");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("failed to read user input");
    match choice {
        &"r" => RSP_CHOICE::Rock,
        &"s" => RSP_CHOICE::Scissors,
        &"p" => RSP_CHOICE::Paper,
        _ => get_user_choice()
    }
}

// randomly generate an RSP choice
fn get_ai_choice() -> RSP_CHOICE {
    let num = rand::thread_rng().gen_range(0..2);
    match num {
        0 => RSP_CHOICE::Rock,
        1 => RSP_CHOICE::Paper,
        2 => RSP_CHOICE::Scissors
    }
}

// compare user and ai choices and declare win / loss
fn get_game_result(user_choice: RSP_CHOICE, ai_choice: RSP_CHOICE) -> GAME_RESULT {
    match user_choice {
        RSP_CHOICE::Rock => {
            match ai_choice {
                RSP_CHOICE::Rock => GAME_RESULT::Tie,
                RSP_CHOICE::Paper => GAME_RESULT::Loss,
                RSP_CHOICE::Scissors => GAME_RESULT::Win,
            }
        },
        RSP_CHOICE::Paper => {
            match ai_choice {
                RSP_CHOICE::Rock => GAME_RESULT::Win,
                RSP_CHOICE::Paper => GAME_RESULT::Tie,
                RSP_CHOICE::Scissors => GAME_RESULT::Loss,
            }
        },
        RSP_CHOICE::Scissors => {
            match ai_choice {
                RSP_CHOICE::Rock => GAME_RESULT::Loss,
                RSP_CHOICE::Paper => GAME_RESULT::Win,
                RSP_CHOICE::Scissors => GAME_RESULT::Tie,
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

    pub fn update_game_results(&mut self, result: &GAME_RESULT) {
        match result {
            GAME_RESULT::Win => self.wins += 1,
            GAME_RESULT::Loss => self.losses += 1,
            GAME_RESULT::Tie => self.ties += 1,
        }
    }

    pub fn get_win_ratio(&self) -> i8 {
        self.wins - (self.losses + self.ties)
    }

    pub fn update_player_choice_stats(&mut self, choice: &RSP_CHOICE ) {
        match choice {
            RSP_CHOICE::Rock => self.rock_picks += 1,
            RSP_CHOICE::Paper => self.paper_picks_picks += 1,
            RSP_CHOICE::Scissors => self.scissor_picks_picks += 1,
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