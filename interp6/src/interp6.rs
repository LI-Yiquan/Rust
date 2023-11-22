use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] as char {
            //_ => todo!("Copy interp5, update >, <, - op codes to handle repeats."),
            '<' => {
                let count = bytes[i..]
                    .iter()
                    .take_while(|&&b| b as char == '<')
                    .count();
                prog.push(Ops::Left(count));
                i += count - 1;
            },
            '>' => {
                let count = bytes[i..]
                    .iter()
                    .take_while(|&&b| b as char == '>')
                    .count();
                prog.push(Ops::Right(count));
                i += count - 1;
            },
            '+' => {
                let count = bytes[i..]
                    .iter()
                    .take_while(|&&b| b as char == '+')
                    .count() as u8;
                prog.push(Ops::Add(count));
                i += count as usize - 1;
            },
            '-' => {
                let count = bytes[i..]
                    .iter()
                    .take_while(|&&b| b as char == '-')
                    .count() as u8;
                prog.push(Ops::Sub(count));
                i += count as usize - 1;
            },
            '[' => prog.push(Ops::LBrack(usize::MAX)), // Initialize with a placeholder
            ']' => prog.push(Ops::RBrack(usize::MAX)), // Initialize with a placeholder
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (), // Ignore any other characters
        }
        i += 1;
    }
    //todo!("Copy interp5 implemenation of loop caching");

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
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
           // _ => todo!("Copy interp5, but update Ops::Sub, Ops::Left, Ops::Right instruction."),
           Ops::Left(n) => {
            if cc >= n {
                cc -= n;
            } else {
                cc = 0;
            }
        },
        Ops::Right(n) => {
            cc = cc.saturating_add(n);
        },
        Ops::Add(n) => cells[cc] = cells[cc].wrapping_add(n),
        Ops::Sub(n) => cells[cc] = cells[cc].wrapping_sub(n),
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
