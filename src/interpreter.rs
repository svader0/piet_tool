use std::{default, io::stdout};

use crate::color::{self, ColorName, PietColor};
pub struct PietProgram {
    grid: Vec<Vec<PietColor>>,
    stack: Vec<i32>,
    direction_pointer: Direction,
    codel_chooser: Direction,
    position: (i32, i32),
}

impl PietProgram {
    pub fn new(grid: Vec<Vec<PietColor>>, input_string: String) -> Self {
        PietProgram {
            grid,
            stack: Vec::new(),
            direction_pointer: Direction::Right,
            codel_chooser: Direction::Right,
            position: (0, 0),
        }
    }

    // The direction pointer (DP) is what moves along the program to make it run. It can be in any one of the 4 cardinal directions. The direction pointer always starts at the color block containing the upper-left-most codel, and always starts facing right.
    // After it has executed the proper command, it will move on to the next color block that is both:
    // 1. Touching the current color block, and
    // 2. The farthest block in the direction of the direction pointer.
    // If there are multiple blocks that meet these criteria, the DP will move to the one that is farthest in the direction of the codel chooser. The codel chooser (CC) is what determines the color of the next block the DP will move to. It can be in any one of the 4 cardinal directions. The codel chooser always starts at the color block containing the upper-left-most codel, and always starts facing right.

    fn get_color(&self, position: &(i32, i32)) -> PietColor {
        self.grid[position.1 as usize][position.0 as usize]
    }

    pub fn run(&mut self) {}

    fn choose_codel(&mut self) -> Direction {
        match (&self.direction_pointer, &self.codel_chooser) {
            (Direction::Right, Direction::Left) => Direction::Up,
            (Direction::Right, Direction::Right) => Direction::Down,
            (Direction::Down, Direction::Left) => Direction::Right,
            (Direction::Down, Direction::Right) => Direction::Left,
            (Direction::Left, Direction::Left) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Up,
            (Direction::Up, Direction::Left) => Direction::Left,
            (Direction::Up, Direction::Right) => Direction::Right,
            _ => panic!(
                "Invalid codel chooser: {:?} with direction pointer {:?}",
                &self.codel_chooser, &self.direction_pointer
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn to_vector(&self) -> (i32, i32) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Black,
    White,
    Nothing,
    Push,
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    InNumber,
    InChar,
    OutNumber,
    OutChar,
}

impl Command {
    fn get_command(lightness_difference: i8, hue_difference: i8, next_color: PietColor) -> Self {
        if next_color.name == ColorName::Black {
            return Command::Black;
        } else if next_color.name == ColorName::White {
            return Command::White;
        }
        if lightness_difference == 0 && hue_difference == 0 {
            return Command::Nothing;
        }
        match (lightness_difference, hue_difference) {
            (0, 1) => Command::Push,
            (0, 2) => Command::Pop,
            (0, 3) => Command::Add,
            (0, 4) => Command::Subtract,
            (0, 5) => Command::Multiply,
            (0, 6) => Command::Divide,
            (0, 7) => Command::Mod,
            (1, 0) => Command::Not,
            (1, 1) => Command::Greater,
            (1, 2) => Command::Pointer,
            (1, 3) => Command::Switch,
            (1, 4) => Command::Duplicate,
            (1, 5) => Command::Roll,
            (1, 6) => Command::InNumber,
            (1, 7) => Command::InChar,
            (2, 0) => Command::OutNumber,
            (2, 1) => Command::OutChar,
            _ => panic!(
                "Invalid command for : DL{} DH{}",
                lightness_difference, hue_difference
            ),
        }
    }
}
