mod entry;
mod decoder;

use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use crate::decoder::Decoder;
use crate::entry::{build_entries, Entry};

fn main() {
    let input =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt");

    let entries: Vec<(Decoder, Entry)> = build_entries(input.as_str());

    let number_of_digits_from_length: u32 = entries.iter().map(|(_, entry)|entry.get_number_of_digits_from_length()).sum();

    println!("number_of_digits_from_length: {number_of_digits_from_length}");
}
