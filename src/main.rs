use rand::Rng;

fn main() {
    println!("Hello, world!");
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
fn get_user_choice() {}

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

    pub fn update_game_results(&mut self, result: GAME_RESULT) {
        match result {
            GAME_RESULT::Win => self.wins += 1,
            GAME_RESULT::Loss => self.losses += 1,
            GAME_RESULT::Tie => self.ties += 1,
        }
    }

    pub fn get_result_ratio(&self) -> i8 {
        self.wins - self.losses
    }

    pub fn update_player_choice_stats(&mut self, choice: RSP_CHOICE ) {
        match choice {
            RSP_CHOICE::Rock => self.rock_picks += 1,
            RSP_CHOICE::Paper => self.paper_picks_picks += 1,
            RSP_CHOICE::Scissors => self.scissor_picks_picks += 1,
        }
    }
}