use std::collections::HashMap;
use std::io;
use super::parser::Command;
pub struct Interpreter<'a> {
    env: HashMap<&'a str, i64>
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self { env: HashMap::new()}
    }

    pub fn execute(&mut self, commads: Vec<Command<'a>>) {
        for cmd in commads {
            match cmd {
                Command::Assign { name, value } => {
                    let final_value = value.evaluate(&self.env).expect("Execute Error");
                    self.env.insert(name, final_value);
                }

                Command::Input { name } => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let value: i64 = input.trim().parse().expect("Expected number, but string were give");
                    self.env.insert(name, value);
                }

                Command::Print { name } => {
                    if let Some(val) = self.env.get(&name) {
                        println!("{}", val)
                    } else {
                        println!("Runtime Error: variable '{}' is not defined", name);
                    }
                }
            }
        }
    }
}