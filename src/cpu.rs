use crate::memory::Memory;

pub struct CPU {
    pub registers: [i32; 4],
    pub pc: usize, // Program counter
    pub memory: Memory,
}

impl CPU {
    pub fn new(memory_size: usize) -> Self {
        CPU {
            registers: [0; 4],
            pc: 0,
            memory: Memory::new(memory_size),
        }
    }

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

    pub fn input(&mut self, reg: usize, value: i32) {
        self.registers[reg] = value;
    }

    pub fn output(&self, reg: usize) {
        println!("Register {}: {}", reg, self.registers[reg]);
    }
}
