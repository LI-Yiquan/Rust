use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
      //_ => todo!("Copy implemenation from interp3, but initialize LBrack and RBrack to have usize::max_value()"),
        '<' => prog.push(Ops::Left),
        '>' => prog.push(Ops::Right),
        '+' => prog.push(Ops::Add),
        '-' => prog.push(Ops::Sub),
        '[' => prog.push(Ops::LBrack(usize::MAX)), // Initialize with a placeholder
        ']' => prog.push(Ops::RBrack(usize::MAX)), // Initialize with a placeholder
        '.' => prog.push(Ops::Output),
        ',' => prog.push(Ops::Input),
        _ => (), // Ignore any other characters
    }
    }

    // Notice: we drop bmap here, since it isn't needed.
    let mut bstack = vec![];
    //let mut i = 0;
    // todo!("Copy implementation from interp3, but update the LBrack and RBrack ops directly to store the jump information.");
    let mut bracket_pairs = Vec::new();
    // First pass: identify the bracket pairs
    for (pos, token) in prog.iter().enumerate() {
        match token {
            Ops::LBrack(_) => bstack.push(pos),
            Ops::RBrack(_) => {
                if let Some(start_pos) = bstack.pop() {
                    bracket_pairs.push((start_pos, pos));
                }
            }
            _ => (),
        }
    }
    // Second pass: update the LBrack and RBrack ops with jump information
    for (start_pos, end_pos) in bracket_pairs {
        if let Some(Ops::LBrack(ref mut start_jump)) = prog.get_mut(start_pos) {
            *start_jump = end_pos;
        }
        if let Some(Ops::RBrack(ref mut end_jump)) = prog.get_mut(end_pos) {
            *end_jump = start_pos;
        }
    }




    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            //_ => todo!("Copy the implementation from interp3, dropping bmap."),
            Ops::Left => {
                if cc > 0 {
                    cc -= 1;
                }
            },
            Ops::Right => cc += 1,
            Ops::Add => cells[cc] = cells[cc].wrapping_add(1),
            Ops::Sub => cells[cc] = cells[cc].wrapping_sub(1),
            Ops::LBrack(jump_pos) => {
                if cells[cc] == 0 {
                    pc = jump_pos;
                    continue;
                }
            },
            Ops::RBrack(jump_pos) => {
                if cells[cc] != 0 {
                    pc = jump_pos;
                    continue;
                }
            },
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
        }
        pc += 1;
    }
    Ok(())
}
