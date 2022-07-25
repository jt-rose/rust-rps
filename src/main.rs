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
    Loss
}

// stub out major functions
fn get_user_choice() {}

fn get_ai_choice() {}

fn compare_choices() {}

// store info on game stats
pub struct GameStats {
    wins: i8,
    losses: i8,
    rock_picks: i16,
    paper_picks: i16,
    scissor_picks: i16,
}

impl GameStats {
    pub fn new() -> Self {
        Self {
            wins: 0,
            losses: 0,
            rock_picks: 0,
            paper_picks: 0,
            scissor_picks: 0
        }
    }

    pub fn update_game_results(&mut self, result: GAME_RESULT) {
        match result {
            GAME_RESULT::Win => self.wins += 1,
            GAME_RESULT::Loss => self.losses += 1
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