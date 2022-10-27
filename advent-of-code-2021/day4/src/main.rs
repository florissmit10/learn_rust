use std::fs;

mod bingo_card;
mod bingo_game;

use crate::bingo_card::BingoCard;
use crate::bingo_game::BingoGame;

fn main() {
    let input: String =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt");
    let mut input_bound = input.split("\n\n");
    let drawn_numbers: Vec<u16> = input_bound.next().unwrap().split(',').map(|c| c.parse::<u16>().unwrap()).collect();
    let bingo_cards: Vec<BingoCard> = input_bound
        .map(BingoCard::new).collect();
    println!("Input chain: {:?}",&bingo_cards.len());
    let mut bingo_game = BingoGame::new(drawn_numbers, bingo_cards);
    bingo_game.run_game();

    let winner_scores: Vec<u32> = bingo_game.get_winner_scores();

    let first_winner_score = winner_scores.first().unwrap();
    println!("first_winner_score: {:?}",first_winner_score);

    let last_winner_score = winner_scores.last().unwrap();
    println!("last_winner_score: {:?}",last_winner_score);

    let last_card = bingo_game.won_bingo_cards.last().unwrap();

    println!("Last card: \n {last_card}");

}

