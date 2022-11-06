use itertools::Itertools;
use crate::decoder::Decoder;

pub struct Digit {
    pub signal: String,
}

impl Digit {
    fn get_value_from_length(&self) -> Option<u8> {
        match self.signal.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

}

pub struct Entry {
    digit_patterns: Vec<Digit>,
}

impl Entry {
    pub fn new( digit_patterns: Vec<Digit>) -> Self {
        let mut result = Entry {  digit_patterns };
        //result.add_digit_values();

        result
    }
    pub(crate) fn get_number_of_digits_from_length(&self) -> u32 {
        self.digit_patterns.iter().filter_map(Digit::get_value_from_length).count() as u32
    }
}


pub fn build_entries(input: &str) -> Vec<(Decoder, Entry)> {
    input.lines()
        .filter_map(|line| {
            let (signals, digits): (&str, &str) = line.split('|').into_iter().next_tuple()?;

            let digit_patterns = build_digits(digits);
            let signal_patterns= signals.split(' ').map(String::from).collect();

            Some((Decoder::new(signal_patterns),Entry::new(digit_patterns)))
        })
        .collect::<Vec<_>>()
}

pub fn build_digits(input: &str) -> Vec<Digit> {
    input.
        split_whitespace()
        .map(|signal| Digit { signal: signal.to_string() })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_entry() {
        let input = "aaa bbb ccc ddd eee fff ggg hhh iii jjj | aaa bbb ccc ddd\naa bb cc dd ee ff gg hh ii jj | aa bb cc dd";
        let mut entries = build_entries(input).into_iter();

        assert_eq!(entries.len(), 2);

        let first = entries.next().unwrap();
        let second = entries.next().unwrap();

        //assert_eq!(first.signal_patterns.len(), 10);
        assert_eq!(first.digit_patterns.len(), 4);

        //assert_eq!(second.signal_patterns.len(), 10);
        assert_eq!(second.digit_patterns.len(), 4);
    }

    #[test]
    fn test_build_digit() {
        let input = "aaa bbb ccc";
        let mut entries = build_digits(input).into_iter();

        assert_eq!(entries.len(), 3);

        let first = entries.next().unwrap();
        let second = entries.next().unwrap();
        let third = entries.next().unwrap();

        assert_eq!(first.signal, "aaa");
        assert_eq!(second.signal, "bbb");
        assert_eq!(third.signal, "ccc");
    }

    #[test]
    fn test_get_value_based_on_length() {
        let input = "aa bbb cccc ddddddd";
        let entries = build_digits(input)
            .iter()
            .map(|d| d.get_value_from_length())
            .collect_vec();

        let mut entries_iter = entries.iter();

        assert_eq!((&entries).len(), 4);

        let one = entries_iter.next().unwrap().unwrap();
        let seven = entries_iter.next().unwrap().unwrap();
        let four = entries_iter.next().unwrap().unwrap();
        let eight = entries_iter.next().unwrap().unwrap();

        assert_eq!(one, 1);
        assert_eq!(seven, 7);
        assert_eq!(four, 4);
        assert_eq!(eight, 8);
    }

}