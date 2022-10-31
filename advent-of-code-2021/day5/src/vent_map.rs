use std::collections::HashMap;
use crate::vent_line::VentLine;

pub struct VentMap {
    danger_map: HashMap<(u32, u32), u32>,
}

impl VentMap {
    pub fn new() -> Self {
        VentMap { danger_map: HashMap::new() }
    }
    pub fn process_point(&mut self, vent_line: VentLine) {
        for vent_point in vent_line.get_all_points() {
            let key = (vent_point.x, vent_point.y);
            *self.danger_map.entry(key).or_insert(0) += 1;
        }
    }

    pub fn get_number_of_dangerous_points(&self) -> usize {
        self.danger_map.iter().filter(|(_, val)| **val > 1).count()
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    fn get_danger_level(map: &VentMap, x: u32, y: u32) -> u32 {
        match map.danger_map.get(&(x,y)) {
            None => 0,
            Some(d) => *d,
        }
    }

    fn get_total_danger_level(map: &VentMap) -> u32 {
        map.danger_map.values().sum()
    }

    #[test]
    fn test_single_vent_line() {
        let mut map = VentMap::new();

        map.process_point(VentLine::new("1,2 -> 1,4"));

        assert_eq!(get_danger_level(&map, 1,2), 1);
        assert_eq!(get_danger_level(&map, 1,3), 1);
        assert_eq!(get_danger_level(&map, 1,4), 1);

        assert_eq!(get_total_danger_level(&map, ), 3);
    }

    #[test]
    fn test_multiple_vent_line() {
        let mut map = VentMap::new();

        map.process_point(VentLine::new("1,2 -> 1,4"));
        map.process_point(VentLine::new("1,3 -> 4,3"));

        assert_eq!(get_danger_level(&map, 1,2), 1);
        assert_eq!(get_danger_level(&map, 1,3), 2);
        assert_eq!(get_danger_level(&map, 1,4), 1);

        assert_eq!(get_danger_level(&map, 2,3), 1);
        assert_eq!(get_danger_level(&map, 3,3), 1);
        assert_eq!(get_danger_level(&map, 4,3), 1);

        assert_eq!(map.get_number_of_dangerous_points(), 1);
        assert_eq!(get_total_danger_level(&map, ), 7);
    }

    #[test]
    fn test_multiple_overlap() {
        let mut map = VentMap::new();

        map.process_point(VentLine::new("1,2 -> 1,4"));
        map.process_point(VentLine::new("1,3 -> 1,6"));

        assert_eq!(get_danger_level(&map, 1,2), 1);
        assert_eq!(get_danger_level(&map, 1,3), 2);
        assert_eq!(get_danger_level(&map, 1,4), 2);
        assert_eq!(get_danger_level(&map, 1,5), 1);
        assert_eq!(get_danger_level(&map, 1,6), 1);

        assert_eq!(map.get_number_of_dangerous_points(), 2);
        assert_eq!(get_total_danger_level(&map, ), 7);
    }

    #[test]
    fn test_multiple_reverse() {
        let mut map = VentMap::new();

        map.process_point(VentLine::new("1,4 -> 1,2"));
        map.process_point(VentLine::new("1,6 -> 1,3"));

        assert_eq!(get_danger_level(&map, 1,2), 1);
        assert_eq!(get_danger_level(&map, 1,3), 2);
        assert_eq!(get_danger_level(&map, 1,4), 2);
        assert_eq!(get_danger_level(&map, 1,5), 1);
        assert_eq!(get_danger_level(&map, 1,6), 1);

        assert_eq!(map.get_number_of_dangerous_points(), 2);
        assert_eq!(get_total_danger_level(&map, ), 7);
    }


    #[test]
    fn test_multiple_reverse_overlap() {
        let mut map = VentMap::new();

        map.process_point(VentLine::new("1,1 -> 4,4"));
        map.process_point(VentLine::new("4,4 -> 1,1"));
        map.process_point(VentLine::new("1,5 -> 4,2"));
        map.process_point(VentLine::new("4,2 -> 1,5"));

        assert_eq!(get_danger_level(&map, 1,1), 2);
        assert_eq!(get_danger_level(&map, 2,2), 2);
        assert_eq!(get_danger_level(&map, 3,3), 4);
        assert_eq!(get_danger_level(&map, 4,4), 2);

        assert_eq!(get_danger_level(&map, 1,5), 2);
        assert_eq!(get_danger_level(&map, 2,4), 2);
        assert_eq!(get_danger_level(&map, 4,2), 2);

        assert_eq!(map.get_number_of_dangerous_points(), 7);
    }
}