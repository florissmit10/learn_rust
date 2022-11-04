mod entry;
mod decoder;

use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use crate::entry::{build_entries, Entry};

fn main() {
    let input =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt");

    let entries = build_entries(input.as_str());

    let number_of_digits_from_length: u32 = entries.iter().map(Entry::get_number_of_digits_from_length).sum();

    println!("number_of_digits_from_length: {number_of_digits_from_length}");
}
