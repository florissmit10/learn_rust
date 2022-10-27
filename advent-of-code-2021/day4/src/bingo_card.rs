use std::fmt;
use std::fmt::{Debug, Display};

use prettytable::{Cell, Row, Table};

#[derive(Eq, PartialEq, Debug)]
pub struct BingoCardField {
    number: u16,
    crossed: bool,
}

impl BingoCardField {
    pub fn cross(&mut self, number: u16) {
        if self.number == number {
            self.crossed = true;
        }
    }
}

impl Display for BingoCardField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self.crossed {
            true => String::from("X"),
            false => self.number.to_string()
        };
        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
pub struct BingoCard {
    numbers: Vec<Vec<BingoCardField>>,
    winning_number: Option<u16>,
}

impl Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create the table
        let mut table = Table::new();
        let winning_number= match self.winning_number {
            None => String::from("None"),
            Some(num) => num.to_string(),
        };
        let score = match self.get_final_score() {
            None => String::from("None"),
            Some(score) => score.to_string(),
        };
        writeln!(f, "Winning #: {}", winning_number).unwrap();
        writeln!(f, "score: {}", score).unwrap();

        for bingo_row in &self.numbers {
            let bingo_row: Vec<Cell> = bingo_row.iter().map(|r| Cell::new(format!("{}", r).as_str())).collect();
            table.add_row(Row::new(bingo_row));
        }

        Display::fmt(&table, f)
    }
}

impl BingoCard {
    pub fn new(input: &str) -> Self {
        let numbers: Vec<Vec<BingoCardField>> =
            input.split('\n').map(|s|
                s.split(' ').filter(|n| !n.is_empty())
                    .map(|n| BingoCardField {
                        number: n.trim().parse().unwrap(),
                        crossed: false,
                    }).collect())
                .collect();
        BingoCard { numbers, winning_number: None }
    }

    pub fn cross(&mut self, number: u16){
        if self.winning_number.is_some() {return;}

        for row in &mut self.numbers{
            for field in row{
                if field.number == number{
                    field.cross(number);
                }
            }
        }
        if self.has_bingo() {
            self.winning_number = Some(number);
        }
    }

    pub fn is_crossed(&self, x: usize, y: usize) -> Result<bool, &'static str> {
        self.numbers
            .get(y).ok_or("Cant find index {y}")?
            .get(x).ok_or("Cant find index {x}").map(|bcf| bcf.crossed)
    }

    pub fn has_bingo(&self) -> bool {
        if self.winning_number.is_some() {return true;}

        if self.has_horizontal_bingo(){
            return true;
        }

        if self.has_vertical_bingo(){
            return true;
        }

        //if self.has_diagonal_bingo(){
        //    return true;
       // }
        false
    }

    fn has_horizontal_bingo(&self) -> bool {
        self.numbers.iter().any(|row| row.iter().all(|bcf| bcf.crossed))
    }

    fn has_vertical_bingo(&self) -> bool {
        for i in 0..self.numbers.len(){
            let has_vertical_bingo: bool =
                self.numbers.iter()
                    .map(|v| v.get(i).unwrap())
                    .all(|bcf| bcf.crossed);
            if has_vertical_bingo{
                return true;
            }
        }
        false
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
        if has_bingo {  return true;}
        false
    }

    fn get_sum_of_uncrossed_numbers(&self) -> u16 {
        self.numbers.iter().flatten().map(|bcf| if !bcf.crossed {bcf.number} else {0}).sum()
    }

    pub fn get_final_score(&self) -> Option<u32> {
        self.winning_number.map(|winning_number| u32::from(self.get_sum_of_uncrossed_numbers()) * u32::from(winning_number))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_card_cross() {
        let input = "73 96 47 14 10\n28 11 79 84 20\n74 30  0 59 71\n80 93 42 22 17\n44  2 81 29 15";

        let mut card = BingoCard::new(input);
        let crossed_before = card.is_crossed(0, 0).unwrap();
        card.cross(73);
        let crossed_after = card.is_crossed(0, 0).unwrap();

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
        assert_eq!(card.has_bingo(), true);
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
        assert_eq!(card.has_bingo(), true);
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
        assert_eq!(card.has_bingo(), true);
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
        assert_eq!(card.has_bingo(), true);
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

        assert_eq!(card.get_final_score(), Some(20 * 70))
    }
}