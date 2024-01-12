
use std::collections::HashMap;
use std::io::{Read, self};

pub fn interpret(content: Vec<u8>, dimension: usize) -> (usize, Vec<u8>) {
    let size: usize  = dimension*dimension;

    let mut jumps: HashMap<usize, usize> = HashMap::new();
    let mut jump_stack: Vec<usize> = Vec::new();

    //first pass
    let mut i: usize = 0;
    while i < content.len() {
        match content[i] {
            b'>' | b'<' | b'^' | b'v' | b'V' | b'+' | b'-' | b',' | b'.' | b'!' | b'\n' | b' ' | b'\t' => (),
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
                        eprintln!("Error char {i}: unmatched ] bracket");
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
    if !jump_stack.is_empty() {
        eprintln!("Error char {}: unmatched [ bracket ", jump_stack.pop().unwrap_or_else(|| 0));
        std::process::exit(1);
    }
    let mut mem: Vec<u8> = vec![0u8; size];
    let mut ptr: usize = 0;
    let mut buffer = [0; 1]; //buffer for reading a character in
    //second pass
    i = 0;
    while i < content.len() {
        match content[i] {
            b'>' => ptr = (ptr + 1) % size,
            b'<' => {
                if ptr == 0 {
                    ptr = size - 1;
                } else {
                    ptr = (ptr - 1) % size;
                }
            }
            b'^' => {
                if ptr < dimension {
                    ptr = size - dimension + ptr;
                } else {
                    ptr = (ptr - dimension) % size;
                }
            }
            b'v' | b'V' => ptr = (ptr + dimension) % size,
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
            b'!' => for (i, char) in mem.iter().enumerate() {
                if i != 0 && i % dimension == 0 {println!()}
                print!("{}", if *char == 0 {' '} else {*char as char});
            }
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
    return (ptr, mem);
}
