# Root_VM

## Simple Virtual Machine in Rust

## Overview

This project implements a basic virtual machine (VM) in Rust, consisting of a CPU and a memory management system. The VM is designed to execute a small set of arithmetic instructions. This modular approach ensures the project is easy to extend and maintain.

## How It Works

The virtual machine consists of two main components:

## CPU:

- Registers: Four general-purpose registers (R0, R1, R2, R3) for storing intermediate values.
- Program Counter (PC): A register that points to the current instruction in memory.
- Memory: Interacts with the memory module to fetch and execute instructions.
- Instruction Set: The CPU supports basic arithmetic operations including addition, subtraction, multiplication, and division.

# Memory:

- Data Storage: A simple vector-based memory to store integer values representing instructions and data.
- Read/Write Operations: Methods to read from and write to specific addresses in memory.

## Features

- Modular Design: The CPU and memory are implemented as separate modules, making the codebase more maintainable and extensible.
- Basic Arithmetic Instructions: The VM supports the following operations:
- Addition (ADD)
- Subtraction (SUB)
- Multiplication (MUL)
- Division (DIV)
- Program Execution: The CPU fetches instructions from memory and executes them in a loop until a halt instruction (HALT) is encountered.
- Input/Output Operations: The VM can initialize registers with specific values and output the contents of registers.
- Workflow

## Initialization:

The CPU and memory are initialized with a specified size.
A program (a series of instructions) is loaded into memory.

## Execution:

The CPU runs the program by fetching instructions from memory.
The instructions are executed, modifying the state of the registers.
Output:

The result of the computations is printed to the console.

## Example

In the provided example, the VM:

- Loads a program that performs addition and multiplication.
- Initializes the registers with values.
- Executes the instructions.
- Outputs the final result.
