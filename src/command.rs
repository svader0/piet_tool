use crate::interpreter::PietProgram;
use core::panic;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    pub fn get_command(lightness_difference: i8, hue_difference: i8) -> Self {
        match (lightness_difference, hue_difference) {
            (0, 1) => Self::Add,
            (0, 2) => Self::Divide,
            (0, 3) => Self::Greater,
            (0, 4) => Self::Duplicate,
            (0, 5) => Self::InChar,
            (1, 0) => Self::Push,
            (1, 1) => Self::Subtract,
            (1, 2) => Self::Mod,
            (1, 3) => Self::Pointer,
            (1, 4) => Self::Roll,
            (1, 5) => Self::OutNumber,
            (2, 0) => Self::Pop,
            (2, 1) => Self::Multiply,
            (2, 2) => Self::Not,
            (2, 3) => Self::Switch,
            (2, 4) => Self::InNumber,
            (2, 5) => Self::OutChar,
            _ => panic!(
                "Invalid command for : DL{} DH{}",
                lightness_difference, hue_difference
            ),
        }
    }

    pub fn execute(&self, context: &mut PietProgram) {
        match self {
            Self::Push => {
                let value = context.get_current_value();
                context.stack.push(value);
            }
            Self::Pop => {
                context.stack.pop();
            }
            Self::Add => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(a + b);
            }
            Self::Subtract => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(b - a);
            }
            Self::Multiply => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(a * b);
            }
            Self::Divide => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(b / a);
            }
            Self::Mod => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(b % a);
            }
            Self::Not => {
                let a = context.stack.pop();
                context.stack.push(if a == 0 { 1 } else { 0 });
            }
            Self::Greater => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(if b > a { 1 } else { 0 });
            }
            Self::Pointer => {
                let mut a = context.stack.pop();
                while a != 0 {
                    if a > 0 {
                        context.move_pointer_clockwise();
                    } else {
                        context.move_pointer_anticlockwise();
                    }
                    a -= 1;
                }
            }
            Self::Switch => {
                let mut a = context.stack.pop();
                while a != 0 {
                    context.toggle_codel_chooser();
                    a -= 1;
                }
            }
            Self::Duplicate => {
                let a = context.get_current_value();
                context.stack.push(a);
                context.stack.push(a);
            }
            Self::Roll => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                if b < 0 {
                    return;
                }
                let mut rolls = a % b;
                if rolls < 0 {
                    rolls += b;
                }
                context.stack.roll(b, rolls);
            }
            Self::InNumber => {
                let value = Self::get_input_number();
                context.stack.push(value);
            }
            Self::InChar => {
                let value = Self::get_input_char();
                context.stack.push(value as i32);
            }
            Self::OutNumber => {
                let value = context.stack.pop();
                Self::output_number(value);
            }
            Self::OutChar => {
                let value = context.stack.pop();
                Self::output_char(value as u8);
            }
            _ => panic!("Command not implemented: {:?}", self),
        }
    }

    fn output_char(value: u8) {
        print!("{}", value as char);
    }

    fn output_number(value: i32) {
        print!("{}", value);
    }

    fn get_input_char() -> u8 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.chars().next().unwrap() as u8
    }

    fn get_input_number() -> i32 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }
}
