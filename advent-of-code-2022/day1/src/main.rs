use std::fs;
use std::str::FromStr;
use crate::elf::{Elf, ElfExpedition};

mod elf;

fn main() {
    let expedition = read_input();
    let max_food_elf = expedition.get_elf_with_most_food();
    let max_food = max_food_elf.get_total_food();
    println!("max food: {max_food}");

    let max_3_food_elves = expedition.get_top_n_elves(3);
    let max_3_food: u32 = max_3_food_elves.iter()
        .map(Elf::get_total_food)
        .sum();
    println!("max food in 3 elves: {max_3_food}");
}

fn read_input() -> ElfExpedition {
    let input =
        fs::read_to_string("input.txt").unwrap();

    ElfExpedition::from_str(input.as_str()).unwrap()

}
