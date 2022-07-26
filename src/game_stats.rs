use crate::game_result::GameResult;
use crate::rsp_choice::RspChoice;

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
            scissor_picks: 0,
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
