use std::fs;
use std::io::Error;

mod instruction;
use instruction::{Instruction, Direction};

fn main() {
    let input =read_data().unwrap();

    let mut pos_p1 = Position::new();
    pos_p1.process_instructions_p1(&input);

    println!("Part 1: final depth * position{:?}", pos_p1.depth * pos_p1.horizontal_position);

    let mut pos_p2 = Position::new();
    pos_p2.process_instructions_p2(&input);

    println!("Part 2: final depth * position{:?}", pos_p2.depth * pos_p2.horizontal_position);
}

fn read_data() -> Result<Vec<Instruction>, Error> {
    let input = fs::read_to_string("input.txt")?
        .lines()
        .map(|s|Instruction::new(s))
        .collect::<Vec<_>>();
    Ok(input)
}

#[derive(Debug)]
struct Position{
    depth: i32,
    horizontal_position: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Position{
            depth: 0,
            horizontal_position: 0,
            aim: 0,
        }
    }

    fn process_instruction_p2(&mut self, instruction: &Instruction){
        match instruction {
            Instruction { direction: Direction::Up, distance } => {self.aim -= distance}
            Instruction { direction: Direction::Down, distance } => {self.aim += distance}
            Instruction { direction: Direction::Forward, distance } => {
                self.horizontal_position += distance;
                self.depth += self.aim * distance;
            }
        }
    }

    fn process_instructions_p2(&mut self, instructions: &Vec<Instruction>){
        for i in instructions {
            self.process_instruction_p2(i);
        }
    }

    fn process_instruction_p1(&mut self, instruction: &Instruction){
        match instruction {
            Instruction { direction: Direction::Up, distance } => {self.depth -= distance}
            Instruction { direction: Direction::Forward, distance } => {self.horizontal_position += distance}
            Instruction { direction: Direction::Down, distance } => {self.depth += distance}
        }
    }

    fn process_instructions_p1(&mut self, instructions: &Vec<Instruction>){
        for i in instructions {
            self.process_instruction_p1(i);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_forward() {
        let mut pos = Position::new();
        let instruction = Instruction::new("forward 10");

        pos.process_instruction_p1(&instruction);

        assert_eq!(pos.depth, 0);
        assert_eq!(pos.horizontal_position, 10);
    }

    #[test]
    fn test_p1_instruction_up() {
        let mut pos = Position::new();
        let instruction = Instruction::new("up 2");

        pos.process_instruction_p1(&instruction);

        assert_eq!(pos.depth, -2);
        assert_eq!(pos.horizontal_position, 0);
    }

    #[test]
    fn test_p1_instruction_down() {
        let mut pos = Position::new();
        let instruction = Instruction::new("down 4");

        pos.process_instruction_p1(&instruction);

        assert_eq!(pos.depth, 4);
        assert_eq!(pos.horizontal_position, 0);
    }


    #[test]
    fn test_p1_instructions() {
        let mut pos = Position::new();
        let instructions = vec![Instruction::new("down 4"), Instruction::new("down 4"), Instruction::new("forward 4")];

        pos.process_instructions_p1(&instructions);

        assert_eq!(pos.depth, 8);
        assert_eq!(pos.horizontal_position, 4);
    }


    #[test]
    fn test_p2_forward() {
        let mut pos = Position::new();
        let instruction = Instruction::new("forward 10");

        pos.process_instruction_p1(&instruction);

        assert_eq!(pos.depth, 0);
        assert_eq!(pos.horizontal_position, 10);
    }

    #[test]
    fn test_p2_instruction_up() {
        let mut pos = Position::new();
        let instruction = Instruction::new("up 2");

        pos.process_instruction_p1(&instruction);

        assert_eq!(pos.depth, 2);
        assert_eq!(pos.horizontal_position, 0);
    }

    #[test]
    fn test_p2_instruction_down() {
        let mut pos = Position::new();
        let instruction = Instruction::new("down 4");

        pos.process_instruction_p2(&instruction);

        assert_eq!(pos.depth, 4);
        assert_eq!(pos.horizontal_position, 0);
    }


    #[test]
    fn test_p2_instructions() {
        let mut pos = Position::new();
        let instructions = vec![Instruction::new("down 4"), Instruction::new("down 4"), Instruction::new("forward 4")];

        pos.process_instructions_p2(&instructions);

        assert_eq!(pos.depth, 8);
        assert_eq!(pos.horizontal_position, 4);
    }

}