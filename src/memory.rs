//! Simple memory crate for the virtual machine.

/// A simple memory module for the virtual machine.
pub struct Memory {
    pub data: Vec<i32>,
}

impl Memory {
    /// Creates a new memory instance with the specified size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the memory to be allocated.
    ///
    /// # Examples
    ///
    /// ```
    /// use root_vm::memory::Memory;
    /// let memory = Memory::new(256);
    /// assert_eq!(memory.data.len(), 256);
    /// ```
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    /// Reads a value from the specified memory address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read the value from.
    ///
    /// # Examples
    ///
    /// ```
    /// use root_vm::memory::Memory;
    /// let mut memory = Memory::new(256);
    /// memory.write(0, 42);
    /// assert_eq!(memory.read(0), 42);
    /// ```
    pub fn read(&self, address: usize) -> i32 {
        self.data[address]
    }

    /// Writes a value to the specified memory address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to write the value to.
    /// * `value` - The value to be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use root_vm::memory::Memory;
    /// let mut memory = Memory::new(256);
    /// memory.write(0, 42);
    /// assert_eq!(memory.read(0), 42);
    /// ```
    pub fn write(&mut self, address: usize, value: i32) {
        self.data[address] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn test_memory_new() {
        let memory = Memory::new(256);
        assert_eq!(memory.data.len(), 256);
        assert_eq!(memory.read(0), 0); // Ensure all values are initialized to 0
    }

    #[test]
    fn test_memory_read_write() {
        let mut memory = Memory::new(256);
        memory.write(0, 42);
        assert_eq!(memory.read(0), 42);
        memory.write(100, 99);
        assert_eq!(memory.read(100), 99);
    }

    #[test]
    #[should_panic]
    fn test_memory_out_of_bounds() {
        let memory = Memory::new(256);
        memory.read(256); // This should panic
    }
}
