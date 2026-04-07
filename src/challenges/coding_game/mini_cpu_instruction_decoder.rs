use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct CPU {
    R0: Register,
    R1: Register,
    R2: Register,
    R3: Register,
}

struct Register {
    instruction: u8
}

enum Instruction {
    MOV,
    ADD,
    SUB,
    MUL,
    INC,
    DEC,
    HLT
}

impl Instruction {
    fn parse_instruction(instruction: &str) -> Self {
        match instruction {
            "01" => Instruction::MOV,
            "02" => Instruction::ADD,
            "03" => Instruction::SUB,
            "04" => Instruction::MUL,
            "05" => Instruction::INC,
            "06" => Instruction::DEC,
            "07" => Instruction::MOV,
            "FF" => Instruction::HLT,
            _ => panic!("Unsupported instruction {instruction}")
        }
    }
}

/**
 * Decode and execute the MiniCPU bytecode program.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let program = input_line.trim_matches('\n').to_string(); // Space-separated hex bytes representing CPU instructions

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");


    // Print the final value of each register R0, R1, R2, R3, one value per line
    println!("answer");
}
