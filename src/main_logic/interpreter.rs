use std::collections::HashMap;
use std::io;
use super::parser::Command;
pub struct Interpreter {
    env: HashMap<String, i32>
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: HashMap::new()}
    }

    pub fn execute(&mut self, commads: Vec<Command>) {
        for cmd in commads {
            match cmd {
                Command::Assign { name, value } => {
                    let final_value = value.evaluate(&self.env);
                    self.env.insert(name, final_value);
                }

                Command::Input { name } => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let value: i32 = input.trim().parse().expect("Expected number, but string were give");
                    self.env.insert(name, value);
                }

                Command::Print { name } => {
                    if let Some(val) = self.env.get(&name) {
                        println!("{}", val)
                    } else {
                        panic!("Runtime Error: variable '{}' is not defined", name);
                    }
                }
            }
        }
    }
}