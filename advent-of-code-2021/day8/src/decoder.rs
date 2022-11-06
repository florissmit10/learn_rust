use std::collections::HashMap;
use crate::entry::Digit;

pub struct Decoder {
    signals: Vec<String>,
    digit_map: HashMap<String, char>,
    pattern_length_map: HashMap<u8, Vec<String>>,
}
fn get_pattern_length_map(signals: &Vec<String>) -> HashMap<u8, Vec<String>> {
    signals.iter()
        .fold(HashMap::new(), |mut acc, s| {
            let mut entry =acc.entry(s.len() as u8).or_insert(vec![]);
            entry.push(s.clone());
            acc
        })
}

impl Decoder {
    pub fn new(signals: Vec<String>) -> Self {
        let digit_map = HashMap::new();

        let pattern_length_map= get_pattern_length_map(&signals);
        let mut result = Decoder{signals, digit_map, pattern_length_map};
        result.init_decoder();
        result
    }

    fn init_decoder(&mut self) {
        self.init_length_based();
        let _length_six_patterns = self.pattern_length_map.get(&6).unwrap();

        //length_six_patterns

    }

    fn init_length_based(&mut self) {
        self.digit_map.insert(self.pattern_length_map.get(&2).unwrap().first().unwrap().clone(), '1');
        self.digit_map.insert(self.pattern_length_map.get(&4).unwrap().first().unwrap().clone(), '4');
        self.digit_map.insert(self.pattern_length_map.get(&3).unwrap().first().unwrap().clone(), '7');
        self.digit_map.insert(self.pattern_length_map.get(&8).unwrap().first().unwrap().clone(), '8');
    }


    pub fn decode(&self, _digit_signals: Vec<Digit>) -> u32 {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input: Vec<String> = vec!["acedgfb","cdfbe","gcdfa","fbcad","dab","cefabd","cdfgeb","eafb","cagedb","ab"]
            .iter().map(|s|String::from(*s)).collect();

        let decoder = Decoder::new(input);

        let mut expected_digit_map:HashMap<String, char> = HashMap::new();
        expected_digit_map.insert(String::from("abcdefg"), '8');
        expected_digit_map.insert(String::from("bcdef"), '5');
        expected_digit_map.insert(String::from("acdfg"), '2');
        expected_digit_map.insert(String::from("abcdf"), '3');
        expected_digit_map.insert(String::from("abd"), '7');
        expected_digit_map.insert(String::from("abcdef"), '9');
        expected_digit_map.insert(String::from("abef"), '4');
        expected_digit_map.insert(String::from("abcdeg"), '0');
        expected_digit_map.insert(String::from("ab"), '1');
        assert_eq!(expected_digit_map, decoder.digit_map);

    }

}