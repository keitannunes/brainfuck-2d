use std::fs;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::io::{self, Read};

const DIMENSION: i32 = 8;
const SIZE: usize = (DIMENSION * DIMENSION) as usize;

fn main() -> io::Result<()> {
    //get file content
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let content: Vec<u8> = fs::read_to_string(path)?.into_bytes();

    let mut jumps: HashMap<usize, usize> = HashMap::new();
    let mut jump_stack: Vec<usize> = Vec::new();

    //first pass
    let mut i: usize = 0;
    while i < content.len() {
        match content[i] {
            b'>' | b'<' | b'+' | b'-' | b',' | b'.' | b'\n' | b' ' | b'\t' => (), //do nothing
            b'[' => {
                jump_stack.push(i);
            }
            b']' => {
                match jump_stack.pop() {
                    Some(x) => {
                        jumps.insert(x, i);
                        jumps.insert(i, x);
                        i += 1;
                    }
                    None => {
                        eprintln!("char {i}: unmatched ] bracket");
                        std::process::exit(1)
                    }
                }
            }
            _ => { //comment
                let comment_start: usize = i;
                while i < content.len() && content[i] != b'\n' { i += 1; }
                jumps.insert(comment_start, i);
            }
        }
        i += 1
    }

    let mut mem: [u8; SIZE] = [0; SIZE];
    let mut ptr: usize = 0;
    let mut buffer = [0; 1]; //buffer for reading a character in
    //second pass
    i = 0;
    while i < content.len() {
        match content[i] {
            b'>' => ptr = (ptr + 1) % SIZE,
            b'<' => {
                if ptr == 0 {
                    ptr = SIZE - 1;
                } else {
                    ptr = (ptr - 1) % SIZE;
                }
            }
            b'+' => mem[ptr] = mem[ptr].wrapping_add(1),
            b'-' => mem[ptr] = mem[ptr].wrapping_sub(1),
            b',' => {
                match io::stdin().read_exact(&mut buffer) {
                    Ok(_) => {
                        let character = buffer[0];
                        if character.is_ascii() {
                            mem[ptr] = character;
                        } else {
                            eprintln!("Runtime Error: Non-ASCII character entered.");
                            std::process::exit(1);
                        }
                    }
                    Err(error) => println!("Error reading input: {}", error),
                }
            }
            b'.' => print!("{}", mem[ptr] as char),
            b'[' | b']' => {
                if mem[ptr] != 0 && content[i] == b']' || mem[ptr] == 0 && content[i] == b'[' {
                    match jumps.get(&i) {
                        Some(x) => i = *x,
                        None => {
                            eprintln!("Internal Error: Jump at {i} not found");
                            std::process::exit(1);
                        }
                    }
                }
            }
            b'\n' | b'\t' | b' ' => (),
            _ => { //comment
                match jumps.get(&i) {
                    Some(x) => i = *x,
                    None => {
                        eprintln!("Internal Error: Jump at {i} not found");
                        std::process::exit(1);
                    }
                }
            }
        }
        i += 1;
    }
    println!("{:?}", mem);
    Ok(())
}