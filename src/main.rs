use root_vm::cpu::CPU;

fn main() {
    let mut cpu = CPU::new(256);

    // Load a program into memory
    cpu.memory.write(0, 1); // ADD instruction
    cpu.memory.write(1, 0); // Register 0
    cpu.memory.write(2, 1); // Register 1
    cpu.memory.write(3, 2); // Register 2
    cpu.memory.write(4, 3); // MUL instruction
    cpu.memory.write(5, 0); // Register 0
    cpu.memory.write(6, 0); // Register 0
    cpu.memory.write(7, 3); // Register 3
    cpu.memory.write(8, 0); // HALT

    // Initialize registers for the test
    cpu.input(1, 10);
    cpu.input(2, 20);
    cpu.input(3, 2);

    // Run the program
    cpu.run();

    // Output the result
    cpu.output(0); // Should output 60 (10 + 20) * 2
}
