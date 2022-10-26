use std::{fmt, fs};
use std::fmt::{Debug, Display};


use prettytable::{Table, Row, Cell};

fn main() {
    let input =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt");
    let mut input_bound = input.split("\n\n");
    let drawn_numbers: Vec<u16> = input_bound.next().unwrap().split(",").map(|c| c.parse::<u16>().unwrap()).collect();
    let bingo_cards: Vec<BingoCard> = input_bound
        .map(|s| BingoCard::new(s)).collect();
    let bingo_game = BingoGame { drawn_numbers, bingo_cards };
    let (winning_number, remaining_numbers) = bingo_game.run_game().unwrap();
    let winning_scores: Vec<u32> = remaining_numbers.iter().map(|n| u32::from(*n) * u32::from(winning_number)).collect();
    println!("winning_scores: {:?}",winning_scores);
}

#[derive(Debug)]
struct BingoCardField {
    number: u16,
    crossed: bool,
}

impl BingoCardField {
    fn cross(&mut self, number: u16) {
        if self.number == number {
            self.crossed = true;
        }
    }
}

impl Display for BingoCardField {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self.crossed {
            true => String::from("X"),
            false => self.number.to_string()
        };
        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
struct BingoCard {
    numbers: Vec<Vec<BingoCardField>>,
}

impl Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create the table
        let mut table = Table::new();

        for bingo_row in &self.numbers {
            let bingo_row: Vec<Cell> = bingo_row.iter().map(|r| Cell::new(format!("{}", r).as_str())).collect();
            table.add_row(Row::new(bingo_row));
        }

        Display::fmt(&table, f)
    }
}

impl BingoCard {
    fn new(input: &str) -> Self {
        let numbers: Vec<Vec<BingoCardField>> =
            input.split("\n").map(|s|
                s.split(' ').filter(|n| !n.is_empty())
                    .map(|n| BingoCardField {
                        number: n.trim().parse().unwrap(),
                        crossed: false,
                    }).collect())
                .collect();
        BingoCard { numbers }
    }

    fn cross(&mut self, number: u16){
        for row in &mut self.numbers{
            for field in row{
                if field.number == number{
                   field.cross(number);
                }
            }
        }
    }

    fn is_crossed(&self, x: usize, y: usize) -> Result<bool, &'static str> {
        self.numbers
            .get(y).ok_or("Cant find index {y}")?
            .get(x).ok_or("Cant find index {x}")
            .and_then(|bcf| Ok(bcf.crossed))
    }

    fn has_bingo(&self) -> bool {

        if self.has_horizontal_bingo(){
            return true;
        }

        if self.has_vertical_bingo(){
            return true;
        }

        if self.has_diagonal_bingo(){
            return true;
        }
        return false;
    }

    fn has_horizontal_bingo(&self) -> bool {
        self.numbers.iter().any(|row| row.iter().all(|bcf| bcf.crossed))
    }

    fn has_vertical_bingo(&self) -> bool {
        for i in 0..self.numbers.len(){
            let has_vertical_bingo: bool =
                (&self.numbers).iter()
                    .map(|v| v.get(i).unwrap())
                    .all(|bcf| bcf.crossed);
            if has_vertical_bingo{
                return true;
            }
        }
        return false;
    }

    fn has_diagonal_bingo(&self) -> bool {
        let mut has_bingo = true;
        for i in 0..self.numbers.len(){
            has_bingo = has_bingo && self.numbers.get(i).unwrap().get(i).unwrap().crossed
        }
        if has_bingo { return true;}

        has_bingo = true;
        for i in 0..self.numbers.len(){
            has_bingo = has_bingo && self.numbers.get(i).unwrap().get(self.numbers.len()-1 - (i)).unwrap().crossed
        }
        if has_bingo { return true; }
        return false;
    }

    fn get_sum_of_uncrossed_numbers(&self) -> u16 {
        self.numbers.iter().flatten().map(|bcf| if !bcf.crossed {bcf.number} else {0}).sum()
    }
}

#[derive(Debug)]
struct BingoGame {
    drawn_numbers: Vec<u16>,
    bingo_cards: Vec<BingoCard>,
}

impl BingoGame {
    fn run_game(mut self) -> Option<(u16, Vec<u16>)> {
        for number in self.drawn_numbers{
            for mut card in &mut self.bingo_cards {
                card.cross(number);
            }
            if self.bingo_cards.iter().any(|c| c.has_bingo()){
                let winner_scores: Vec<u16> = self.bingo_cards.iter().filter(|c| c.has_bingo()).map(|c| c.get_sum_of_uncrossed_numbers()).collect();
                return Some((number, winner_scores));
            }
        }
        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_card_cross() {
        let input = "73 96 47 14 10\n28 11 79 84 20\n74 30  0 59 71\n80 93 42 22 17\n44  2 81 29 15";

        let mut card = BingoCard::new(input);
        let crossed_before = card.is_crossed(0,0).unwrap();
        card.cross(73);
        let crossed_after = card.is_crossed(0,0).unwrap();

        assert_eq!(crossed_before, false);
        assert_eq!(crossed_after, true);
        //assert!(false)
    }

    #[test]
    fn test_bingo_card_horizontal() {
        let input = "73 96 47 14 10\n28 11 79 84 20\n74 30  0 59 71\n80 93 42 22 17\n44  2 81 29 15";

        let mut card = BingoCard::new(input);
        assert_eq!(card.has_bingo(), false);
        card.cross(28);
        card.cross(11);
        card.cross(79);
        card.cross(84);
        card.cross(20);
        assert_eq!( card.has_bingo(), true);
        assert_eq!(card.has_horizontal_bingo(), true);
        assert_eq!(card.has_vertical_bingo(), false);
        assert_eq!(card.has_diagonal_bingo(), false);
    }

    #[test]
    fn test_bingo_card_vertical() {
        let input = "73 96 47 14 10\n28 11 79 84 20\n74 30  0 59 71\n80 93 42 22 17\n44  2 81 29 15";

        let mut card = BingoCard::new(input);
        assert_eq!(card.has_bingo(), false);
        card.cross(14);
        card.cross(84);
        card.cross(59);
        card.cross(22);
        card.cross(29);
        assert_eq!( card.has_bingo(), true);
        assert_eq!(card.has_horizontal_bingo(), false);
        assert_eq!(card.has_vertical_bingo(), true);
        assert_eq!(card.has_diagonal_bingo(), false);
    }

    #[test]
    fn test_bingo_card_diagonal() {
        let input = "73 96 47 14 10\n28 11 79 84 20\n74 30  0 59 71\n80 93 42 22 17\n44  2 81 29 15";

        let mut card = BingoCard::new(input);
        assert_eq!(card.has_bingo(), false);
        card.cross(73);
        card.cross(11);
        card.cross(0);
        card.cross(22);
        card.cross(15);
        assert_eq!( card.has_bingo(), true);
        assert_eq!(card.has_horizontal_bingo(), false);
        assert_eq!(card.has_vertical_bingo(), false);
        assert_eq!(card.has_diagonal_bingo(), true);
    }

    #[test]
    fn test_bingo_card_diagonal_other() {
        let input = "73 96 47 14 10\n28 11 79 84 20\n74 30  0 59 71\n80 93 42 22 17\n44  2 81 29 15";

        let mut card = BingoCard::new(input);
        assert_eq!(card.has_bingo(), false);
        card.cross(10);
        card.cross(84);
        card.cross(0);
        card.cross(93);
        card.cross(44);
        assert_eq!( card.has_bingo(), true);
        assert_eq!(card.has_horizontal_bingo(), false);
        assert_eq!(card.has_vertical_bingo(), false);
        assert_eq!(card.has_diagonal_bingo(), true);
    }

    #[test]
    fn test_sum_of_uncrossed() {
        let input = "10 20\n30 40";

        let mut card = BingoCard::new(input);

        assert_eq!(card.get_sum_of_uncrossed_numbers(), 100);

        card.cross(10);
        assert_eq!(card.get_sum_of_uncrossed_numbers(), 90);

        card.cross(20);
        assert_eq!(card.get_sum_of_uncrossed_numbers(), 70);

        card.cross(30);
        assert_eq!(card.get_sum_of_uncrossed_numbers(), 40);

        card.cross(40);
        assert_eq!(card.get_sum_of_uncrossed_numbers(), 0);
    }
}