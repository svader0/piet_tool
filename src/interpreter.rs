use std::ops::Add;

use clap::builder::styling::Color;

use crate::{
    color::{ColorName, PietColor},
    command::Command,
};
#[derive(Debug)]
pub struct PietProgram {
    // The Piet program is a 2D grid of codels, each of which is a color.
    grid: Vec<Vec<PietColor>>,
    // The stack is a LIFO data structure that holds integers. Piet is a stack-based language.
    stack: Vec<i32>,
    // The DP is the direction pointer. It points in one of four directions: right, down, left, or up.
    direction_pointer: Direction,
    // The CC is the codel chooser. It points in one of two directions: right or left.
    codel_chooser: Direction,
    // The position is the current position of the interpreter in the grid.
    position: (i32, i32),
    // The current value is the current value of the color block that our interpreter is on.
    current_value: i32,
}

impl PietProgram {
    pub fn new(grid: Vec<Vec<PietColor>>, input_string: String) -> Self {
        // convert each character in the input string to its ASCII value
        // and put it on the stack
        let mut stack = Vec::new();
        for x in input_string.chars() {
            let ascii = x as i32;
            stack.push(ascii);
        }
        PietProgram {
            grid,
            stack: stack,
            direction_pointer: Direction::Right,
            codel_chooser: Direction::Right,
            position: (0, 0),
            current_value: 0,
        }
    }

    // getters and setters

    pub fn get_color(&self, position: &(i32, i32)) -> PietColor {
        self.grid[position.1 as usize][position.0 as usize]
    }

    pub fn get_current_value(&self) -> i32 {
        self.current_value
    }

    pub fn toggle_codel_chooser(&mut self) {
        self.codel_chooser = match self.codel_chooser {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            _ => panic!("Invalid codel chooser: {:?}", &self.codel_chooser),
        };
    }

    pub fn move_pointer_clockwise(&mut self) {
        self.direction_pointer = match self.direction_pointer {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        };
    }

    pub fn move_pointer_anticlockwise(&mut self) {
        self.direction_pointer = match self.direction_pointer {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        };
    }

    pub fn get_stack(&self) -> &Vec<i32> {
        &self.stack
    }

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap() // TODO: Handle this error
    }

    pub fn roll(&mut self, depth: i32, rolls: i32) {
        let depth = depth.abs() as usize;
        let rolls = rolls.abs() as usize;
        let rolls = rolls % self.stack.len();
        let depth = depth % self.stack.len();
        let mut stack = self.stack.clone();
        let mut rolled = Vec::new();
        for i in 0..depth {
            rolled.push(stack.pop().unwrap());
        }
        for i in 0..rolls {
            let value = rolled.pop().unwrap();
            stack.insert(depth, value);
        }
        self.stack = stack;
    }

    pub fn execute(&mut self) {
        let mut terminate = false;

        loop {
            // check to see if we've terminated the program
            // which only happens if we've reached a black codel or an edge and we've
            // tried to move 8 times and failed. (See encounter_edge)
            if terminate == true {
                println!("Terminating program");
                break;
            }
            // Move our position to the next codel
            self.step();
            self.current_value = self.get_codels().len() as i32;
            // Get the color of the current codel
            let current_color = self.get_color(&self.position);
            // check bounds
            if self.next_is_edge() {
                terminate = self.encounter_edge();
                continue;
            }

            // Get the color of the next codel
            let next_pos = self.get_next_position().unwrap();
            let next_color = self.get_color(&next_pos);
            // check if white
            if next_color.name == ColorName::White {
                self.glide();
                continue;
            }

            // Get the difference in lightness and hue between the current and next codels
            let lightness_difference = current_color.lightness_difference(&next_color);
            let hue_difference = current_color.hue_difference(&next_color);
            // Get the command for the current and next codels
            let command = Command::get_command(lightness_difference, hue_difference, next_color);
            println!(
                "Command \" {:?} \" chosen based on transition from {:?} to {:?} at position {:?} with lightness difference {} and hue difference {}",
                command, current_color.name, next_color.name, self.position, lightness_difference, hue_difference
            );
            self.position = next_pos;
            command.execute(self);
        }
    }

    fn next_is_edge(&self) -> bool {
        let next_pos = self.get_next_position();
        next_pos.is_none()
            || next_pos.unwrap().0 < 0
            || next_pos.unwrap().0 >= self.grid[0].len() as i32
            || next_pos.unwrap().1 < 0
            || next_pos.unwrap().1 >= self.grid.len() as i32
    }

    // Black colour blocks and the edges of the program restrict program flow.
    // If the Piet interpreter attempts to move into a black block or off an edge, it is stopped and the CC is toggled.
    // The interpreter then attempts to move from its current block again. If it fails a second time, the DP is moved clockwise one step.
    // These attempts are repeated, with the CC and DP being changed between alternate attempts.
    // If after eight attempts the interpreter cannot leave its current colour block, there is no way out and the program terminates.
    fn encounter_edge(&mut self) -> bool {
        let mut attempts = 0;
        loop {
            let next_pos = self.get_next_position();
            if self.next_is_edge() {
                attempts += 1;
                println!("Attempt: {}", attempts);
                if attempts == 8 {
                    return true;
                }
                if attempts % 2 == 0 {
                    self.codel_chooser = match self.codel_chooser {
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                        _ => panic!("Invalid codel chooser: {:?}", &self.codel_chooser),
                    };
                } else {
                    self.direction_pointer = match self.direction_pointer {
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                    };
                }
            } else {
                self.position = next_pos.unwrap();
                return false;
            }
        }
    }

    // If the DP encounters a white codel, it will glide along the white codels until it reaches a colored codel.
    fn glide(&mut self) {
        loop {
            let next_pos = self.get_next_position();
            if next_pos.is_none() {
                self.encounter_edge();
            }
            let next_pos = next_pos.unwrap();
            if next_pos.0 < 0
                || next_pos.0 >= self.grid[0].len() as i32
                || next_pos.1 < 0
                || next_pos.1 >= self.grid.len() as i32
                || self.get_color(&next_pos).name != ColorName::White
            {
                break;
            }
            self.position = next_pos;
        }
    }

    fn step(&mut self) {
        // Given all the codels in the color block, get all codels in the current color block that are on the furthest edge in the direction of the DP.
        // For example, if the DP is facing right, get all codels on the FARTHEST right edge of the color block.
        let all_codels = self.get_codels();
        // find max x or y values depending on DP direction
        let max: i32 = match self.direction_pointer {
            Direction::Right => all_codels.iter().map(|c| c.0).max().unwrap(),
            Direction::Down => all_codels.iter().map(|c| c.1).max().unwrap(),
            Direction::Left => all_codels.iter().map(|c| c.0).min().unwrap(),
            Direction::Up => all_codels.iter().map(|c| c.1).min().unwrap(),
        };

        // using our max, The interpreter finds the codel of the current colour block on that edge which is furthest to the CC's direction of the DP's direction of travel.
        // For example, if our direction_pointer is right and our codel chooser is left, we choose the uppermost codel along the max edge (the codel with the smallest y value)
        let codel = all_codels
            .iter()
            .filter(|c| match self.direction_pointer {
                Direction::Right => c.0 == max,
                Direction::Down => c.1 == max,
                Direction::Left => c.0 == max,
                Direction::Up => c.1 == max,
            })
            .min_by_key(|c| match self.codel_chooser {
                Direction::Left => c.1,
                Direction::Right => c.1,
                _ => panic!("Invalid codel chooser: {:?}", &self.codel_chooser),
            })
            .unwrap();
        self.position = *codel;
    }

    // finds the edge of the current colour block which is furthest in the direction of the DP. (This edge may be disjoint if the block is of a complex shape.)
    // returns all codels in the current color block
    fn get_codels(&self) -> Vec<(i32, i32)> {
        let mut codels = Vec::new();
        let mut visited = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        let mut stack = vec![self.position];
        let current_color = self.get_color(&self.position); // Get the color of the current position
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            if visited[current.1 as usize][current.0 as usize] {
                continue;
            }
            visited[current.1 as usize][current.0 as usize] = true;
            if self.get_color(&current) == current_color {
                // Only add codels with the same color as the current position
                codels.push(current);
            }
            for direction in &[
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Up,
            ] {
                let next_position = (
                    current.0 + direction.to_vector().0,
                    current.1 + direction.to_vector().1,
                );
                if next_position.0 >= 0
                    && next_position.0 < self.grid[0].len() as i32
                    && next_position.1 >= 0
                    && next_position.1 < self.grid.len() as i32
                    && self.get_color(&next_position) == current_color
                // Only push positions with the same color as the current position
                {
                    stack.push(next_position);
                }
            }
        }
        codels
    }

    fn get_next_position(&self) -> Option<(i32, i32)> {
        if (self.position.0 < 0
            || self.position.0 >= self.grid[0].len() as i32
            || self.position.1 < 0
            || self.position.1 >= self.grid.len() as i32)
        {
            return None;
        } else {
            let (dx, dy) = self.direction_pointer.to_vector();
            let next_position = (self.position.0 + dx, self.position.1 + dy);
            Some(next_position)
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
