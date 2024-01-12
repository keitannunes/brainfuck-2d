use std::collections::HashMap;
use std::io::{Read, self};
use std::fmt;
#[derive(Debug, Clone)]
pub struct InterpreterError {
    location: usize,
    reason: String,
}
impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "char {}: {} ", self.location, self.reason)
    }
}
impl std::error::Error for InterpreterError {}


pub fn interpret(content: Vec<u8>, dimension: usize) -> Result<(usize, Vec<u8>), InterpreterError> {
    let size: usize  = dimension*dimension;
    let mut jumps: HashMap<usize, usize> = HashMap::new();
    let mut jump_stack: Vec<usize> = Vec::new();

    //first pass
    let mut i: usize = 0;
    while i < content.len() {
        match content[i] {
            b'>' | b'<' | b'^' | b'v' | b'V' | b'+' | b'-' | b',' | b'.' | b'!' | b'\n' | b' ' | b'\t' | b'\r' => (),
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
                        return Err(InterpreterError {location: i, reason: String::from("Unmatched ] bracket")})
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
        return Err(InterpreterError {location: jump_stack.pop().unwrap_or_else(|| 0), reason: String::from("Unmatched [ bracket")});
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
                            return Err(InterpreterError {location: i, reason: String::from("Non-ASCII character entered")})
                        }
                    }
                    Err(e) => return Err(InterpreterError {
                        location: i,
                        reason: format!("{} (This issue is likely caused by using the ',' command after using stdin to send the program.",e)
                    })
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
                            return Err(InterpreterError {location: i, reason: String::from("Jump not found")})
                        }
                    }
                }
            }
            b'\n' | b'\t' | b' ' | b'\r' => (),
            _ => { //comment
                match jumps.get(&i) {
                    Some(x) => i = *x,
                    None => {
                        return Err(InterpreterError {location: i, reason: String::from("Jump not found")})
                    }
                }
            }
        }
        i += 1;
    }
    return Ok((ptr, mem));
}
