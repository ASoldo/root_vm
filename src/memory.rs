pub struct Memory {
    pub data: Vec<i32>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    pub fn read(&self, address: usize) -> i32 {
        self.data[address]
    }

    pub fn write(&mut self, address: usize, value: i32) {
        self.data[address] = value;
    }
}
