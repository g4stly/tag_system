extern crate serde_json;
extern crate serde;

use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Production {
    t: char,
    w: String
}

#[derive(Deserialize, Debug)]
struct Definition {
    init: String,
    token_size: usize,
    halt_symbol: char,
    productions: Vec<Production>
}

struct System {
    buffer: String,
    token_size: usize,
    halt_symbol: char,
    alphabet: HashMap<char, String>
}

impl System {
    fn new(data: &str) -> Result<System, io::Error> {
        let definition: Definition = serde_json::from_str(data)?;
        let mut system = System {
            alphabet: HashMap::new(),
            buffer: definition.init.clone(),
            token_size: definition.token_size,
            halt_symbol: definition.halt_symbol
        };

        for rule in &definition.productions {
            system.alphabet.insert(rule.t, rule.w.clone());
        }

        Ok(system)
    }

    fn step(&mut self) -> Option<String> {
        if self.buffer.chars().count() < self.token_size { return None; }
        let mut tail = self.buffer.split_off(self.token_size);
        match self.buffer.chars().next() {
            Some(c) => {
                if c == self.halt_symbol { 
                    return None; 
                }

                if let Some(word) = self.alphabet.get(&c) {
                        tail.push_str(word.as_str());
                } else {
                        panic!("unexpected token {}", c);
                }

                self.buffer = tail;
                Some(self.buffer.clone())
            },
            None => {
                None
            }
        }
    }

    fn run(mut self) {
        let mut generation = 0;
        println!("{:03X}: {}", generation, self.buffer);
        loop {
            if let Some(buf) = self.step() {
                generation += 1;
                println!("{:03X}: {}", generation, buf);
            } else {
                break;
            }
        }
    }
}

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    let args: Vec<String> = env::args().collect(); 

    if args.len() < 2 {
        // read from stdin like a good unix boy
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut input)?;
    } else {
        File::open(args[1].as_str())?
            .read_to_string(&mut input)?;
    }

    Ok(input)
}

fn main() {
    // read from file or stdin
    let input: String;
    match read_input() {
        Ok(i) => {
            input = i;
        },
        Err(e) => {
            eprintln!("failed to read input: {}", e);
            return;
        }
    }

    // branch on successful initialization
    match System::new(input.as_str()) {
        Ok(s) => {
            s.run();
        },
        Err(ref e) if e.kind() == io::ErrorKind::InvalidData => {
            eprintln!("definition syntax error: {}", e);
            return;
        },
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
}
