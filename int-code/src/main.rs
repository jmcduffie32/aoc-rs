use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct ProgramState {
    index: usize,
    memory: Vec<i32>,
}

impl ProgramState {
    fn value_at_index(&self, index: usize) -> i32 {
        if self.memory[index] >= 0 {
            self.memory[self.memory[index] as usize]
        } else {
            println!("Invalid program state");
            println!("{:?}", self);
            panic!();
        }

    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("day_2_input.txt")?;
    let mut instructions = String::new();
    file.read_to_string(&mut instructions)?;
    let mut program_state = load_program(&instructions);
    'outer: for i in 0..99 {
        for j in 0..99 {
            program_state = load_program(&instructions);
            program_state.memory[1] = i;
            program_state.memory[2] = j;

            run_program(&mut program_state);
            if program_state.memory[0] == 19690720 {
                break 'outer;
            }
        }
    }
    println!("{}", program_state.memory[0]);
    println!("{}", 100 * program_state.memory[1] + program_state.memory[2]);
    Ok(())
}

fn load_program(instructions: &String) -> ProgramState {
    let memory: Vec<i32> = instructions
        .split(",")
        .map(|ch| ch.parse().unwrap())
        .collect();
    ProgramState{
        index: 0,
        memory
    }
}

fn run_program(program_state: &mut ProgramState) {
    while program_state.memory[program_state.index] != 99 {
        process_op_code(program_state.memory[program_state.index], program_state);
    }
}

fn process_op_code(op_code: i32, program_state: &mut ProgramState) {
    let current_index = program_state.index;
    match op_code {
        1 => {
            let dest_index = program_state.memory[current_index + 3] as usize;
            let arg_1 = program_state.value_at_index(current_index + 1);
            let arg_2 = program_state.value_at_index(current_index + 2);
            // println!("{} {} {}", dest_index, arg_1, arg_2);
            program_state.memory[dest_index] = arg_1 + arg_2;
            program_state.index = current_index + 4;
        },
        2 => {
            let dest_index = program_state.memory[current_index + 3] as usize;
            let arg_1 = program_state.value_at_index(current_index + 1);
            let arg_2 = program_state.value_at_index(current_index + 2);
            program_state.memory[dest_index] = arg_1 * arg_2;
            program_state.index = current_index + 4;
        },
        _ => {
            println!("{:?}", program_state);
            panic!("Unknown opcode:");
        }
    }

}
