use std::collections::HashMap;
use crate::entry::Digit;

pub struct Decoder {
    signals: Vec<String>,
    digit_map: HashMap<String, char>,
    pattern_length_map: HashMap<u8, Vec<String>>,
}

impl Decoder {
    pub fn new(signals: Vec<String>) -> Self {
        let digit_map = HashMap::new();

        let pattern_length_map: HashMap<u8, Vec<String>> = signals.iter()
            .fold(HashMap::new(), |mut acc, s| {
                let mut entry =acc.entry(s.len() as u8).or_insert(vec![]);
                entry.push(s.clone());
                acc
            });

        return Decoder{signals, digit_map, pattern_length_map}
    }

    pub fn decode(&self, digit_signals: Vec<Digit>) -> u32 {
        0
    }
}