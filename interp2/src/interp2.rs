use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;

    // "b" is for bracket
    // let mut bmap = vec![]; // Map from a position in the program to the jump location
    //let initial_capacity = 80000000;
    //let mut bmap = HashMap::with_capacity(initial_capacity);
    let mut bstack = vec![]; // Used to track nested brackets
    /*todo!("Build the bracket map by preprocessing the program.");*/
    let mut jump_map = vec![0; prog.len()]; // Vector to store jump positions

    // Build the jump map by preprocessing the program
    for (pos, &token) in prog.iter().enumerate() {
        match token as char {
            '[' => bstack.push(pos),
            ']' => {
                if let Some(start_pos) = bstack.pop() {
                    jump_map[start_pos] = pos;
                    jump_map[pos] = start_pos;
                }
            }
            _ => (),
        }
    }
    

    let mut pc = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] as char {
      /*_ => todo!("Copy your source code from interp1. You'll need to modify the `[` and `]` instructions."),*/
        '<' => {
            if cc > 0 {
                cc -= 1;
            }
        },
        '>' => cc += 1,
        '+' => cells[cc] = cells[cc].wrapping_add(1),
        '-' => cells[cc] = cells[cc].wrapping_sub(1),
        '[' => if cells[cc] == 0 { pc = jump_map[pc]; },
        ']' => if cells[cc] != 0 { pc = jump_map[pc]; },
        '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
        ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
        _ => (), /* Ignore any other characters */
            
    }
        pc += 1;
    }
    Ok(())
}
