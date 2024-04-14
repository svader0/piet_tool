use crate::color::PietColor;
pub struct PietProgram {
    grid: Vec<Vec<PietColor>>,
    codel_size: i32,
    input_string: String,
    stack: Vec<i32>,
    direction: Direction,
    position: (i32, i32),
    output: String,
    output_file: String,
    translate: bool,
}

pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl PietProgram {
    pub fn new(
        grid: Vec<Vec<PietColor>>,
        codel_size: i32,
        input_string: String,
        output_file: String,
        translate: bool,
    ) -> Self {
        PietProgram {
            grid,
            codel_size,
            input_string,
            stack: Vec::new(),
            direction: Direction::Right,
            position: (0, 0),
            output: String::new(),
            output_file,
            translate,
        }
    }
}
