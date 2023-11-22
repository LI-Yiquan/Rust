use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack,
    RBrack,
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    /* Notice: prog is now a vec of OpCodes, not a string */
    let mut prog = vec![];

    /* First parse the program into a sequence of opcodes */
    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
            //_ => todo!("Translate all of the commands into their opcodes."),
            '<' => prog.push(Ops::Left),
            '>' => prog.push(Ops::Right),
            '+' => prog.push(Ops::Add),
            '-' => prog.push(Ops::Sub),
            '[' => prog.push(Ops::LBrack),
            ']' => prog.push(Ops::RBrack),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (), // Ignore any other characters
        }
    }

    let mut pc = 0;
    //let mut bmap = vec![];
    //let initial_capacity = 8000000;
    //let mut bmap = HashMap::with_capacity(initial_capacity);
    let mut bstack = vec![];
    //todo!("Copy implementation from interp2, but match on opcodes instead of characters.");

    let mut jump_map = vec![0; prog.len()]; // Vector to store jump positions

    // Build the jump map by preprocessing the program
    for (pos, op) in prog.iter().enumerate() {
        match op {
            Ops::LBrack => bstack.push(pos),
            Ops::RBrack => {
                if let Some(start_pos) = bstack.pop() {
                    jump_map[start_pos] = pos;
                    jump_map[pos] = start_pos;
                }
            }
            _ => (),
        }
    }
    
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] {
            //_ => todo!(
            //    "Copy implementation from interp2, but match on opcodes instead of characters."
            //),
            Ops::Left => if cc > 0 { cc -= 1; },
            Ops::Right => cc += 1,
            Ops::Add => cells[cc] = cells[cc].wrapping_add(1),
            Ops::Sub => cells[cc] = cells[cc].wrapping_sub(1),
            Ops::LBrack => if cells[cc] == 0 { pc = jump_map[pc]; },
            Ops::RBrack => if cells[cc] != 0 { pc = jump_map[pc]; },
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            
        }
        pc += 1;
    }
    Ok(())
}
