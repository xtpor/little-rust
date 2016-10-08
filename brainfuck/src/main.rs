
use std::io;
use std::io::prelude::*;
use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::fmt::Display;

use std::collections::HashMap;


struct Intrepreter {
    code: String,
    jump_table: HashMap<usize, usize>,
    memory: [u8; 30000],
    prog_ptr: usize,
    mem_ptr: usize,
    eof: bool,
}

impl Intrepreter {
    fn initiate (code: String) -> Result<Intrepreter, &'static str> {
        let code = oneline_code(code);
        try!(verify_code_characters(&code));
        let jump_table = try!(build_jump_table(&code));
        Ok(Intrepreter {
            code: code,
            jump_table: jump_table,
            memory: [0; 30000],
            prog_ptr: 0,
            mem_ptr: 0,
            eof: false,
        })
    }

    fn execute_single (&mut self) -> char {
        let mut next_inc = self.prog_ptr + 1;
        if self.is_halted() {
            return 0 as char;
        }
        let instruction = self.code.as_bytes()[self.prog_ptr] as char;
        match instruction {
            '>' => self.mem_ptr = add_mod(self.mem_ptr as i32, 1, 30000) as usize,
            '<' => self.mem_ptr = add_mod(self.mem_ptr as i32, -1, 30000) as usize,
            '+' => *self.mem_ref() = add_mod(*self.mem_ref() as i32, 1, 256) as u8,
            '-' => *self.mem_ref() = add_mod(*self.mem_ref() as i32, -1, 256) as u8,
            '.' => putchar(*self.mem_ref() as char),
            ',' => match getchar() {
                Some(ch) => *self.mem_ref() = ch as u8,
                None => self.eof = true,
            },
            '[' => (),
            ']' => if *self.mem_ref() != 0 {
                next_inc = *self.jump_table.get(&self.prog_ptr).unwrap();
            },
            _ => (),
        }
        self.prog_ptr = next_inc;
        instruction
    }

    fn is_halted (&self) -> bool {
        self.prog_ptr == self.code.len() && !self.eof
    }

    fn mem_ref (&mut self) -> &mut u8 {
        &mut self.memory[self.mem_ptr]
    }
}

fn add_mod (a: i32, b: i32, modulo: u32) -> i32 {
    let mut remainder = (a + b) % (modulo as i32);
    if remainder < 0 {
        remainder += modulo as i32;
    }
    remainder
}

fn getchar () -> Option<char> {
    let mut buf = [0; 1];
    match io::stdin().read_exact(&mut buf) {
        Ok(()) => Some(buf[0] as char),
        Err(_) => None,
    }
}

fn putchar (c: char) {
    write!(io::stdout(), "{}", c);
}

fn build_jump_table (oneline_code: &String) -> Result<HashMap<usize, usize>, &'static str> {
    let mut jump_table = HashMap::new();
    let mut bracket_count = 0;
    let mut index_last_open_bracket = 0;
    for (i, c) in oneline_code.chars().enumerate() {
        if bracket_count < 0 {
            return Err("Unbalanced square brackets");
        }
        match c {
            '[' => {
                index_last_open_bracket = i;
                bracket_count += 1;
            },
            ']' => {
                jump_table.insert(i, index_last_open_bracket);
                bracket_count += 1;
            },
            _ => (),
        }
    }
    Ok(jump_table)
}

fn verify_code_characters (oneline_code: &String) -> Result<(), &'static str> {
    let chars_allowed = "><+-.,[]";
    let fulfilled = oneline_code.chars().all(|c| chars_allowed.chars().any(|p| p == c));
    if fulfilled {
        Ok(())
    } else {
        Err("Source file contains invalid character")
    }
}

fn oneline_code (code: String) -> String {
    code.replace(" ", "").replace("\n", "").replace("\t", "")
}

fn unwrap_exit<E, D: Display> (result: Result<E, D>) -> E {
    result.unwrap_or_else(|e| {
        writeln!(io::stderr(), "Error: {}", e);
        process::exit(-1);
    })
}

fn main () {
    let source_path = unwrap_exit(env::args().nth(1).ok_or("Usage: bfi <source-file>"));
    let absolute_path = unwrap_exit(fs::canonicalize(source_path));
    let mut file = unwrap_exit(File::open(absolute_path));
    let mut code = String::new();
    unwrap_exit(file.read_to_string(&mut code));

    let mut intrepreter = unwrap_exit(Intrepreter::initiate(code));
    while !intrepreter.is_halted() {
        intrepreter.execute_single();
    }
}

#[test]
fn add_mod_test () {
    assert_eq!(add_mod(5, -10, 256), 251);
}
