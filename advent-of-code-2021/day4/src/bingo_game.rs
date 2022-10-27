use crate::bingo_card::BingoCard;

#[derive(Debug)]
pub struct BingoGame {
    pub drawn_numbers: Vec<u16>,
    pub playing_bingo_cards: Vec<BingoCard>,
    pub won_bingo_cards: Vec<BingoCard>,
}

impl BingoGame {
    pub fn new(drawn_numbers: Vec<u16>, playing_bingo_cards: Vec<BingoCard>) -> Self {
        BingoGame { drawn_numbers, playing_bingo_cards, won_bingo_cards: vec![] }
    }

    pub fn run_game(&mut self) {
        for number in &self.drawn_numbers{
            let mut won_cards: Vec<usize> = vec![];
            for (i, card) in self.playing_bingo_cards.iter_mut().enumerate() {
                card.cross(*number);

                if card.has_bingo() {
                    won_cards.push(i)
                }
            }
            won_cards.reverse();

            for i in won_cards{
                let card = self.playing_bingo_cards.remove(i);
                self.won_bingo_cards.push(card);
            }
        }
    }

    pub fn get_winner_scores(&self) -> Vec<u32> {
        self.won_bingo_cards.iter().map(|bc| bc.get_final_score().unwrap()).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_games() {
        let input = vec![
            "10 20 30\n40 50 60\n70 80 90",
            "10 21 30\n40 50 60\n70 80 91",
            "10 22 30\n40 50 60\n70 80 92",
            "10 23 30\n40 50 60\n70 80 93",
            "10 24 30\n40 50 60\n70 80 94",
            "10 20 31\n40 50 60\n70 80 95",
            "10 20 32\n40 50 60\n70 80 96",
        ];

        let drawn_numbers = vec![10,30,20,21,22,23,24];

        let playing_bingo_cards: Vec<BingoCard> = input.iter()
            .map(|s| BingoCard::new(s)).collect();

        let mut bingo_game = BingoGame {drawn_numbers, playing_bingo_cards, won_bingo_cards: vec![]};

        bingo_game.run_game();

        let expected_remaining_numbers: Vec<u32> =
            vec![390,391,392,393,394];
        let expected_winning_numbers: Vec<u32> = vec![20,21,22,23,24];

        assert_eq!(bingo_game.playing_bingo_cards.len(), 2);
        assert_eq!(bingo_game.won_bingo_cards.len(), 5);

        for (i,card) in bingo_game.won_bingo_cards.iter().enumerate() {
            assert_eq!(card.get_final_score(), Some(expected_winning_numbers[i] * expected_remaining_numbers[i]));
        }
    }
}