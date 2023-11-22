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
    Zero,
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;

    // Compile
    while i < bytes.len() {
        match bytes[i] as char {
            //_ => todo!("Copy interp6 implementation"),
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

    // Optimize
    /* Iterate through the program, in search of our "Zero" optimization */
    //todo!("Implement Zero Op Code optimization");
    // Optimize for Zero opcode
    // let mut optimized_prog = Vec::new();
    //let optimized_prog = Vec::new();
    let mut i = 0;
    while i < prog.len() {
        if i + 2 < prog.len() &&
           matches!(prog[i], Ops::LBrack(_)) &&
           matches!(prog[i + 1], Ops::Sub(1)) &&
           matches!(prog[i + 2], Ops::RBrack(_)) {
            prog[i] = Ops::Zero;
            prog.drain(i + 1..=i + 2);
        } else {
            i += 1;
        }
    }
    //prog = optimized_prog;
    //todo!("Copy loop caching implementation from interp6.");
    // Notice: we drop bmap here, since it isn't needed.
    let mut bstack = vec![];
    //let mut i = 0;
    // todo!("Copy implementation from interp3, but update the LBrack and RBrack ops directly to store the jump information.");
    // Build the bracket map by preprocessing the program
    let mut jumps = vec![0; prog.len()];
    // First pass: identify the bracket pairs
    for (pos, op) in prog.iter().enumerate() {
        match op {
            Ops::LBrack(_) => bstack.push(pos),
            Ops::RBrack(_) => {
                if let Some(start_pos) = bstack.pop() {
                    jumps[start_pos] = pos;
                    jumps[pos] = start_pos;
                }
            }
            _ => (),
        }
    }

    // Second pass: update the LBrack and RBrack ops with jump information
    for (pos, jump_pos) in jumps.iter().enumerate() {
        match prog[pos] {
            Ops::LBrack(_) => prog[pos] = Ops::LBrack(*jump_pos),
            Ops::RBrack(_) => prog[pos] = Ops::RBrack(*jump_pos),
            _ => (),
        }
    }



    // Interpret / Evaluate
    let mut cells = vec![0u8; 10000];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            //_ => todo!("Copy interp6 implementation, add handling for Zero Op Code"),
            Ops::Zero => cells[cc] = 0,
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
