use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;
    let mut pc = 0; /* Program counter tracks location in the code */
    let mut cells = vec![0u8; 10000]; /* memory */
    let mut cc = 0; /* Cell counter (data pointer) points to active location in memory*/

    while pc < prog.len() {
        match prog[pc] as char {
            /*'<' => todo!(),
            '>' => todo!(),
            '+' => todo!(),
            '-' => todo!(),
            '[' if cells[cc] == 0 => {
                todo!()
            }
            ']' if cells[cc] != 0 => {
                todo!()
            }*/
            '<' => {
                if cc > 0 {
                    cc -= 1;
                }
            },
            '>' => cc += 1,
            '+' => cells[cc] = cells[cc].wrapping_add(1),
            '-' => cells[cc] = cells[cc].wrapping_sub(1),
            '[' => {
                if cells[cc] == 0 {
                    let mut loop_counter = 1;
                    while loop_counter > 0 {
                        pc += 1;
                        match prog[pc] as char {
                            '[' => loop_counter += 1,
                            ']' => loop_counter -= 1,
                            _ => (),
                        }
                    }
                }
            },
            ']' => {
                if cells[cc] != 0 {
                    let mut loop_counter = 1;
                    while loop_counter > 0 {
                        pc -= 1;
                        match prog[pc] as char {
                            ']' => loop_counter += 1,
                            '[' => loop_counter -= 1,
                            _ => (),
                        }
                    }
                }
            },
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
