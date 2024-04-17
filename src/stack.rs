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

    pub fn roll(&mut self, depth: i32, rolls: i32) {
        let depth = depth as usize;
        let rolls = rolls as usize;
        let mut data = self.data.split_off(self.data.len() - depth);
        data.rotate_right(rolls);
        self.data.append(&mut data);
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
