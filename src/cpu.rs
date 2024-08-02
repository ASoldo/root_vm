//! This module defines the CPU for the simple virtual machine.

use crate::memory::Memory;

/// Represents the CPU of the virtual machine.
pub struct CPU {
    pub registers: [i32; 4],
    pub pc: usize, // Program counter
    pub memory: Memory,
}

impl CPU {
    /// Creates a new CPU instance with the specified memory size.
    ///
    /// # Arguments
    ///
    /// * `memory_size` - The size of the memory to be allocated.
    ///
    /// # Examples
    ///
    /// ```
    /// use root_vm::cpu::CPU;
    /// let cpu = CPU::new(256);
    /// assert_eq!(cpu.memory.data.len(), 256);
    /// ```
    pub fn new(memory_size: usize) -> Self {
        CPU {
            registers: [0; 4],
            pc: 0,
            memory: Memory::new(memory_size),
        }
    }

    /// Runs the program loaded into memory.
    ///
    /// This method fetches and executes instructions in a loop until a halt instruction is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use root_vm::cpu::CPU;
    /// let mut cpu = CPU::new(256);
    /// cpu.memory.write(0, 1); // ADD instruction
    /// cpu.memory.write(1, 0); // Register 0
    /// cpu.memory.write(2, 1); // Register 1
    /// cpu.memory.write(3, 2); // Register 2
    /// cpu.memory.write(4, 0); // HALT
    /// cpu.input(1, 10);
    /// cpu.input(2, 20);
    /// cpu.run();
    /// assert_eq!(cpu.registers[0], 30);
    /// ```
    pub fn run(&mut self) {
        loop {
            let opcode = self.memory.read(self.pc);
            println!("PC: {}, Opcode: {}", self.pc, opcode); // Debug statement
            match opcode {
                1 => {
                    // ADD instruction
                    let reg1 = self.memory.read(self.pc + 1) as usize;
                    let reg2 = self.memory.read(self.pc + 2) as usize;
                    let reg3 = self.memory.read(self.pc + 3) as usize;
                    println!("ADD R{} = R{} + R{}", reg1, reg2, reg3); // Debug statement
                    self.registers[reg1] = self.registers[reg2] + self.registers[reg3];
                    self.pc += 4;
                }
                2 => {
                    // SUB instruction
                    let reg1 = self.memory.read(self.pc + 1) as usize;
                    let reg2 = self.memory.read(self.pc + 2) as usize;
                    let reg3 = self.memory.read(self.pc + 3) as usize;
                    println!("SUB R{} = R{} - R{}", reg1, reg2, reg3); // Debug statement
                    self.registers[reg1] = self.registers[reg2] - self.registers[reg3];
                    self.pc += 4;
                }
                3 => {
                    // MUL instruction
                    let reg1 = self.memory.read(self.pc + 1) as usize;
                    let reg2 = self.memory.read(self.pc + 2) as usize;
                    let reg3 = self.memory.read(self.pc + 3) as usize;
                    println!("MUL R{} = R{} * R{}", reg1, reg2, reg3); // Debug statement
                    self.registers[reg1] = self.registers[reg2] * self.registers[reg3];
                    self.pc += 4;
                }
                4 => {
                    // DIV instruction
                    let reg1 = self.memory.read(self.pc + 1) as usize;
                    let reg2 = self.memory.read(self.pc + 2) as usize;
                    let reg3 = self.memory.read(self.pc + 3) as usize;
                    println!("DIV R{} = R{} / R{}", reg1, reg2, reg3); // Debug statement
                    self.registers[reg1] = self.registers[reg2] / self.registers[reg3];
                    self.pc += 4;
                }
                0 => {
                    println!("HALT"); // Debug statement
                    break;
                }
                _ => unimplemented!("Unknown opcode: {}", opcode),
            }
        }
    }

    /// Sets the value of the specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to set the value for.
    /// * `value` - The value to set.
    ///
    /// # Examples
    ///
    /// ```
    /// use root_vm::cpu::CPU;
    /// let mut cpu = CPU::new(256);
    /// cpu.input(1, 10);
    /// assert_eq!(cpu.registers[1], 10);
    /// ```
    pub fn input(&mut self, reg: usize, value: i32) {
        self.registers[reg] = value;
    }

    /// Prints the value of the specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to print the value of.
    ///
    /// # Examples
    ///
    /// ```
    /// use root_vm::cpu::CPU;
    /// let mut cpu = CPU::new(256);
    /// cpu.input(1, 10);
    /// cpu.output(1); // Prints "Register 1: 10"
    /// ```
    pub fn output(&self, reg: usize) {
        println!("Register {}: {}", reg, self.registers[reg]);
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_cpu_new() {
        let cpu = CPU::new(256);
        assert_eq!(cpu.memory.data.len(), 256);
        assert_eq!(cpu.registers, [0; 4]);
    }

    #[test]
    fn test_cpu_input_output() {
        let mut cpu = CPU::new(256);
        cpu.input(1, 42);
        assert_eq!(cpu.registers[1], 42);
        cpu.output(1); // Should print "Register 1: 42"
    }

    #[test]
    fn test_cpu_add() {
        let mut cpu = CPU::new(256);
        cpu.memory.write(0, 1); // ADD instruction
        cpu.memory.write(1, 0); // Register 0
        cpu.memory.write(2, 1); // Register 1
        cpu.memory.write(3, 2); // Register 2
        cpu.memory.write(4, 0); // HALT
        cpu.input(1, 10);
        cpu.input(2, 20);
        cpu.run();
        assert_eq!(cpu.registers[0], 30);
    }

    #[test]
    fn test_cpu_sub() {
        let mut cpu = CPU::new(256);
        cpu.memory.write(0, 2); // SUB instruction
        cpu.memory.write(1, 0); // Register 0
        cpu.memory.write(2, 1); // Register 1
        cpu.memory.write(3, 2); // Register 2
        cpu.memory.write(4, 0); // HALT
        cpu.input(1, 30);
        cpu.input(2, 10);
        cpu.run();
        assert_eq!(cpu.registers[0], 20);
    }

    #[test]
    fn test_cpu_mul() {
        let mut cpu = CPU::new(256);
        cpu.memory.write(0, 3); // MUL instruction
        cpu.memory.write(1, 0); // Register 0
        cpu.memory.write(2, 1); // Register 1
        cpu.memory.write(3, 2); // Register 2
        cpu.memory.write(4, 0); // HALT
        cpu.input(1, 5);
        cpu.input(2, 4);
        cpu.run();
        assert_eq!(cpu.registers[0], 20);
    }

    #[test]
    fn test_cpu_div() {
        let mut cpu = CPU::new(256);
        cpu.memory.write(0, 4); // DIV instruction
        cpu.memory.write(1, 0); // Register 0
        cpu.memory.write(2, 1); // Register 1
        cpu.memory.write(3, 2); // Register 2
        cpu.memory.write(4, 0); // HALT
        cpu.input(1, 20);
        cpu.input(2, 5);
        cpu.run();
        assert_eq!(cpu.registers[0], 4);
    }
}
