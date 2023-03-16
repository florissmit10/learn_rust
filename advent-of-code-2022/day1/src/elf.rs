use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Ord)]
pub struct Elf {
    food_carried: Vec<u32>,
}

impl Elf {
    pub fn get_total_food(&self) -> u32 {
        self.food_carried.iter().sum()
    }
}

impl FromStr for Elf {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let food_carried =
            s.split("\n")
                .filter_map(|n| n.parse::<u32>().ok()).collect();

        Ok(Elf { food_carried })
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let diff: i32 = self.get_total_food() as i32 - other.get_total_food() as i32;
        match diff {
            d if d < 0 => Some(Less),
            d if d > 0 => Some(Greater),
            d if d == 0 => Some(Equal),
            _ => unreachable!()
        }
    }
}

pub struct ElfExpedition {
    elves: Vec<Elf>,
}

impl ElfExpedition {
    pub fn get_elf_with_most_food(&self) -> &Elf {
        self.elves.iter()
            .max_by(|a,b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    pub fn get_top_n_elves(&self, n: usize) -> &[Elf] {
        match n < self.elves.len()  {
            false => &self.elves,
            true => &self.elves[0..n]
        }
    }
}

impl FromStr for ElfExpedition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: Vec<Elf> =
            s.split("\n\n")
                .map(|i|Elf::from_str(i).unwrap())
                .sorted()
                .rev()
                .collect::<Vec<_>>();

        Ok(ElfExpedition { elves })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_from_str() {
        let input = "1\n2\n3";
        let obj = Elf::from_str(input).unwrap();

        assert_eq!(3, obj.food_carried.len());
        assert_eq!(6, obj.get_total_food());
    }


    #[test]
    fn test_elf_ord() {
        let e1 = &Elf::from_str("1").unwrap();
        let e2 = &Elf::from_str("2").unwrap();
        let e3 = &Elf::from_str("2").unwrap();
        let e4 = &Elf::from_str("3").unwrap();



        assert_eq!(e1,e1.min(e2));
        assert_eq!(e4,e3.max(e4));
    }

    #[test]
    fn test_elf_exp_from_str() {
        let input = "1\n2\n3\n\n4\n5\n6\n";
        let obj = ElfExpedition::from_str(input).unwrap();

        assert_eq!(2, obj.elves.len());
        assert_eq!(15, obj.elves.get(0).unwrap().get_total_food());
    }


    #[test]
    fn test_elf_exp_top_n() {
        let input = "1\n1\n1\n\n2\n2\n2\n\n3\n3\n3\n";
        let obj = ElfExpedition::from_str(input).unwrap();

        assert_eq!(3, obj.elves.len());
        assert_eq!(vec![9 as u32, 6 as u32].as_slice(), obj.get_top_n_elves(2).iter().map(Elf::get_total_food).collect::<Vec<u32>>());
    }
}