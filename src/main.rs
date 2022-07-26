mod game_result;
mod game_stats;
mod rsp_choice;
mod game_steps;

use std::{io};
use crate::game_stats::GameStats;
use crate::game_result::GameResult;
use crate::game_steps::{get_ai_choice, get_game_result, get_user_choice};

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

            let round_summary = format!(
                "You played {} and the computer played {}",
                &player_choice, &ai_choice
            );
            let round_result = get_game_result(player_choice, ai_choice);

            // update round stats and provide message
            match round_result {
                GameResult::Win => {
                    rounds_won += 1;
                    println!("{} - You won!", round_summary);
                }
                GameResult::Loss => {
                    rounds_lost += 1;
                    println!("{} - You lost!", round_summary);
                }
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
        io::stdin()
            .read_line(&mut press_continue)
            .expect("failed to read user input");
        if press_continue.trim() == "q" {
            println!("Thanks for playing!");
            keep_playing = false;
        }
    }
}
