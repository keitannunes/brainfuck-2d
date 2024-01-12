use std::fs;
use std::path::Path;
use std::env;
use std::io;
use std::collections::HashMap;

const DIMENSION: i32 = 8;
const SIZE: usize = (DIMENSION * DIMENSION) as usize;

fn main() -> io::Result<()> {
    //get file content
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let content: Vec<u8> = fs::read_to_string(path)?.into_bytes();

    let mut mem: [u8; SIZE] = [0; SIZE];
    let mut jumps: HashMap<usize, usize> = HashMap::new();
    let mut jump_stack: Vec<usize> = Vec::new();
    //first pass
    let mut i: usize = 0;
    while i < content.len() {
            match content[i] {
                b'>' | b'<' | b'+' | b'-' | b',' | b'.' | b'\n' => (), //do nothing
                b'[' => {
                    jump_stack.push(i);
                },
                b']' => {
                    match jump_stack.pop() {
                        Some(x) => {
                            jumps.insert(x,i+1);
                            jumps.insert(i, x+1);
                            i += 1;
                        }
                        None => {
                            eprintln!("char {i}: unmatched ] bracket");
                            std::process::exit(1)
                        }
                    }
                }
                _ => {
                    let comment_start: usize = i;
                    while i < content.len() && content[i] != b'\n' {i += 1;}
                    jumps.insert(comment_start, i + 1);
                }
            }
        i += 1
    }
    println!("{:?}", jumps);
    Ok(())
}