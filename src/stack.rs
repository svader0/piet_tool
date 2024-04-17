#[derive(Debug)]
pub struct Stack {
    data: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> i32 {
        self.data.pop().unwrap_or(0)
    }

    // Pops the top two values off the stack, and then rotates the top Y values on the stack up by X,
    // wrapping values that pass the top around to the bottom of the rolled portion,
    // where X is the first value popped (top of the stack), and Y is the second value popped (second on the stack).
    // (Example: If the stack is currently 1,2,3, with 3 at the top, and then you push 3 and then 1, and then roll, the new stack is 3,1,2.)
    pub fn roll(&mut self, depth: i32, rolls: i32) {
        let depth = depth as usize;
        let rolls = rolls as usize;
        let len = self.data.len();
        if depth > len {
            return;
        }
        let mut rolled = self.data.split_off(len - depth);
        let mid = len - depth + if depth != 0 { rolls % depth } else { 0 };
        rolled.rotate_left(mid);
        self.data.append(&mut rolled);
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
