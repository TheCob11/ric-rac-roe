use ric_rac_roe_game::game::*;
use std::io;

fn prompt_move(g: &Game) -> Coords {
    let mut coords: (u8, u8) = (0, 0);
    let mut input: String;
    let stdin = io::stdin();
    loop {
        input = String::new();
        println!(
            "Player {}, input the row you would like to play in(0, 1, or 2; e.g. 0 for top): ",
            g.player_turn()
        );
        stdin.read_line(&mut input).expect("Failed to read line");
        if let Ok(row @ 0..=2) = input.trim().parse::<u8>() {
            coords.0 = row;
        } else {
            println!("Please enter a value between 0(top) and 2(bottom).");
            continue;
        }
        input = String::new();
        println!(
            "Player {}, input the column you would like to play in(0, 1, or 2; e.g. 0 for left): ",
            g.player_turn()
        );
        stdin.read_line(&mut input).expect("Failed to read line");
        if let Ok(col @ 0..=2) = input.trim().parse::<u8>() {
            coords.1 = col;
        } else {
            println!("Please enter a value between 0(left) and 2(right).");
            continue;
        }
        input = String::new();
        println!(
            "Do you want to put your {} in tile ({},{}) (y or n)? ",
            g.player_turn(),
            coords.0,
            coords.1
        );
        stdin.read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "y" {
            return Coords::build(coords.0, coords.1)
                .expect("Values were bounds checked, so they shouldn't be out of [0,2]");
        } else {
            continue;
        }
    }
}

fn play() {
    let mut g: Game = Game::new();
    loop{
        println!("{}", g);
        match g.play_coords(prompt_move(&g)){
            Ok(None) => continue,
            Ok(Some(GameResult::Tie)) => {
                println!("It's a tie!");
                break;
            }
            Ok(Some(GameResult::Winner(winner))) => {
                println!("{winner} wins!");
                break;
            }
            Err(TurnError::GameOver(_)) => {
                println!("Game is already over?");
                break;
            }
            Err(TurnError::TileFull(value)) => {
                println!("{value} is already in that spot!");
            }
        }
    }
    println!("{g}");
}

fn main() {
    play()
}
