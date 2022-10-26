#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: i32,
}

impl Direction {
    fn new(input: &str) -> Self {
        match input {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Cant parse Direction: {input}")
        }
    }
}

impl Instruction {
    pub fn new(input: &str) -> Self {
        let split: Vec<&str> = input.split_whitespace().collect();
        match split.len() {
            2 => Instruction { direction: Direction::new(split[0]), distance: split[1].parse().unwrap() },
            _ => panic!("Cant make Instruction from {input}"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_forward() {
        let input = "forward 10";

        let result = Instruction::new(input);

        assert_eq!(result.direction, Direction::Forward);
        assert_eq!(result.distance, 10);
    }

    #[test]
    fn test_instruction_up() {
        let input = "up 1";

        let result = Instruction::new(input);

        assert_eq!(result.direction, Direction::Up);
        assert_eq!(result.distance, 1);
    }

    #[test]
    fn test_instruction_down() {
        let input ="down 2";

        let result = Instruction::new(input);

        assert_eq!(result.direction, Direction::Down);
        assert_eq!(result.distance, 2);
    }
}