use std::collections::HashMap;
use super::parser::Command;
use rand::*;
#[allow(dead_code)]
pub struct Interpreter<'a> {
    env: HashMap<&'a str, i64>
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self { env: HashMap::new() }
    }

    pub fn get_marks(&mut self, commands: &[Command<'a>]) {
        let mut marks: HashMap<&'a str, usize> = HashMap::new();
        for (idx, mark) in commands.iter().enumerate() {
            if let Command::Label { name } = mark {
                marks.insert(name, idx);
            }
        }
        self.execute(commands, &marks);
    }

    pub fn execute(&mut self, commands: &[Command<'a>], labels: &HashMap<&'a str, usize>) {
        let mut command_idx = 0;
        while command_idx < commands.len() {
            // Check if new index exists to jump
            if let Some(new_idx) = self.execute_single(&commands[command_idx], labels) {
                command_idx = new_idx;
                continue; 
            }
            command_idx += 1;
        }
    }

    // Function returns Option<usize>. 
    // If Some(idx) was returned — GOTO worked and we need to jump to next index
    fn execute_single(&mut self, cmd: &Command<'a>, labels: &HashMap<&'a str, usize>) -> Option<usize> {
        match cmd {
            Command::Label { .. } => None,
            
            Command::GOTO { label } => {
                match labels.get(label) {
                    Some(idx) => Some(*idx), // Return index where to jump
                    None => {
                        println!("Runtime Error: label '{}' not found", label);
                        None
                    }
                }
            }
            
            Command::Assign { name, value } => {
                let final_value = value.evaluate(&self.env).expect("Execute Error");
                self.env.insert(name, final_value);
                None
            }
            
            Command::Input { name } => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value: i64 = input.trim().parse().expect("Expected number");
                self.env.insert(name, value);
                None
            }
     
            Command::PrintStr(text) => {
                println!("{}", text);
                None
            }

            Command::PrintVar(name) => {
                // Если это переменная, ищем её в env
                if let Some(val) = self.env.get(name) {
                    println!("{}", val)
                } else {
                    println!("Runtime Error: variable '{}' is not defined", name)
                }
                None
            }
            
            Command::IF { left_value, cmp, right_value, body } => {
                let lhs = left_value.evaluate(&self.env);
                let rhs = right_value.evaluate(&self.env);

                let condition = match cmp {
                    '=' => lhs == rhs,
                    '!' => lhs != rhs,
                    '<' => lhs < rhs, // Добавили!
                    '>' => lhs > rhs,
                    _ => unreachable!(),
                };

                if condition {
                    // If condition is true, we start cycle and execute programs one by one
                    for inner_cmd in body {
                        // If any functions, (e.g GOTO) return new index, we immeadiatly jump to eat and break main loop
                        if let Some(jump_idx) = self.execute_single(inner_cmd, labels) {
                            return Some(jump_idx);
                        }
                    }
                }
                None
            }

            Command::Random { name, min, max } => {
                let mut rng = rand::thread_rng();
                let min_val = *min;
                let max_val = *max;

                let random_value: i64 = rng.gen_range(min_val..=max_val);
                
                // Записываем его в наше окружение, как обычный LET
                self.env.insert(name, random_value);
                None // Прыжка нет, возвращаем None
            }
        }
    }
}