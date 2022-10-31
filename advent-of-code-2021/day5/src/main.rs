use std::fs;
use crate::vent_line::VentLine;
use crate::vent_map::VentMap;

mod vent_line;
mod vent_map;

fn main() {
    let input: String =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt");
    let vent_lines: Vec<VentLine> = input
        .lines()
        .map(VentLine::new)
        .collect();

    let mut map_straight = VentMap::new();
    let mut map = VentMap::new();
    let vent_lines_straight: Vec<VentLine> = input
        .lines()
        .map(VentLine::new)
        .filter(|vl| vl.is_straight())
        .collect();

    println!("vent_lines_straight #: {}", vent_lines_straight.len());
    println!("vent_lines #: {}", vent_lines.len());

    for vl in  vent_lines_straight{
        map_straight.process_point(vl);
    }
    for vl in  vent_lines{
        map.process_point(vl);
    }

    let danger_points_straight =  map_straight.get_number_of_dangerous_points();
    println!("number of dangerous points (straight) {danger_points_straight}");

    let danger_points =  map.get_number_of_dangerous_points();
    println!("number of dangerous points (all) {danger_points}");

    //println!("{:?}", keys);
}
