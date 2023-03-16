mod height_map;

use std::fs;
use crate::height_map::HeightMap;

fn main() {
    let points:Vec<Vec<u8>> =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt")
            .lines()
            .map(|l|l.chars().map(|c|c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
            .collect::<Vec<_>>();

    let _map = HeightMap::new( points);
}
