extern crate serde_json;
extern crate serde;

use std::io;
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
    alphabet: HashMap<char, String>
}

impl System {
    fn new(data: &str) -> Result<System, io::Error> {
        //let mut buffer = String::new();
        //File::open(filename)?.read_to_string(&mut buffer)?;

        let definition: Definition = serde_json::from_str(data)?;
        let mut system = System {
            token_size: definition.token_size,
            alphabet: HashMap::new(),
            buffer: definition.init.clone()
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
                match self.alphabet.get(&c) {
                    Some(word) => {
                        tail.push_str(word.as_str());
                    },
                    None => {
                        panic!("unexpected token {}", c);
                    }
                }
                self.buffer = tail;
                Some(self.buffer.clone())
            },
            None => {
                None
            }
        }
    }

    fn run(&mut self) {
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

fn main() {
    // read from stdin like a good unix boy
    let stdin = io::stdin();
    let mut input = String::new();
    if let Err(e) = stdin.lock().read_to_string(&mut input) {
            eprintln!("Failed to read from stdin: {}", e);
            return;
    }

    // branch on successful initialization
    match System::new(input.as_str()) {
        Ok(mut s) => {
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
