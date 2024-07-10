use std::io::{self, Read};

#[derive(Debug)]
pub struct Stack {
    data: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            debug!("Attempted to pop from empty stack. Ignoring.");
            return None;
        }
        self.data.pop()
    }

    // Pops the top two values off the stack, and then rotates the top Y values on the stack up by X,
    // wrapping values that pass the top around to the bottom of the rolled portion,
    // where X is the first value popped (top of the stack), and Y is the second value popped (second on the stack).
    // (Example: If the stack is currently 1,2,3, with 3 at the top, and then you push 3 and then 1, and then roll, the new stack is 3,1,2.)
    pub fn roll(&mut self) {
        // Pop the number of rolls
        let rolls = match self.pop() {
            Some(rolls) => rolls,
            None => {
                return; // Return if there are no values to pop
            }
        };

        // Pop the depth
        let depth = match self.pop() {
            Some(depth) => depth,
            None => {
                self.push(rolls); // Push back the rolls if depth is not available
                return;
            }
        };

        // Check if the depth is valid
        if depth < 0 {
            error!("Invalid depth {} for roll operation", depth);
            return;
        } else if depth == 0 {
            return; // No action needed if depth is 0
        }

        let depth = depth as usize;
        if depth > self.len() {
            error!("Depth {} exceeds stack size", depth);
            return;
        }

        // Calculate the effective number of rolls
        let rolls = rolls % depth as i32;
        let rolls = if rolls < 0 {
            rolls + depth as i32 // Adjust for negative rolls
        } else {
            rolls
        } as usize;

        // Only perform the roll if the number of rolls is non-zero
        if rolls == 0 {
            return;
        }

        // Extract the slice to be rolled
        let len = self.len();
        let mut stack_slice = self.data[len - depth..].to_vec();

        // Perform the roll
        for _ in 0..rolls {
            let value = stack_slice.pop().unwrap(); // Remove the last element
            stack_slice.insert(0, value); // Insert it at the beginning
        }

        // Replace the original slice with the rolled slice
        self.data[len - depth..].copy_from_slice(&stack_slice);
        trace!("Rolled stack: depth {} rolls {}", depth, rolls);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn in_char(&mut self) {
        let mut buffer = [0; 1];
        match io::stdin().read_exact(&mut buffer) {
            Ok(_) => {
                let ch = buffer[0] as i32;
                self.push(ch);
                trace!(
                    "Input character: {} (ASCII value: {})",
                    ch as u8 as char,
                    ch
                );
            }
            Err(e) => {
                error!("Failed to read character input: {}", e);
            }
        }
    }

    pub fn in_number(&mut self) {
        // Read the entire input line
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the input to remove any surrounding whitespace or newlines
                let input = input.trim();

                // Attempt to parse the input as an integer
                match input.parse::<i32>() {
                    Ok(number) => {
                        self.push(number);
                        trace!("Input number: {}", number);
                    }
                    Err(e) => {
                        error!("Failed to parse input as number: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to read number input: {}", e);
            }
        }
    }

    pub fn to_string(&self) -> String {
        self.data
            .iter()
            .map(|x| x.to_string())
            .rev()
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
