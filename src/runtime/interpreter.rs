use std::collections::HashMap;
use crate::frontend::ast::Statement;

use rand::Rng;
#[derive(Debug, PartialEq)]
pub enum Signal<'a> {
    Continue,
    Jump {label: &'a str},
    SkipNext,
    Exit,
}

pub struct Enviroment<'a> {
    map: HashMap<&'a str, i64>
}

impl<'a> Enviroment<'a> {
    pub fn new() -> Self {
        Enviroment { map: HashMap::new() }
    }

    pub fn set(&mut self, key: &'a str, value: i64) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &'a str) -> Result<i64, String> {
        self.map
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Runtime Error: variable '{}' not found!", key))
    }
}

pub struct Interpreter<'a> {
    env: Enviroment<'a>
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self { env: Enviroment::new() }
    }

    // Scan full code to get the positions of labels, which will be used fo GOTO function
    pub fn pre_scan_labels(&mut self, commands: &[Statement<'a>]) -> HashMap<&'a str, usize>{
        let mut marks: HashMap<&'a str, usize> = HashMap::new();
        for (idx, mark) in commands.iter().enumerate() {
            if let Statement::Label { name } = mark {
                marks.insert(name, idx);
            }
        }
        dbg!(&marks);
        marks
        
    }

    pub fn execute(&mut self, commands: &[Statement<'a>], labels: &HashMap<&'a str, usize>) -> Result<(), String> {
        let mut command_idx = 0;
        while command_idx < commands.len() {
            match self.execute_single(&commands[command_idx])? {
                Signal::Exit => {
                    break;
                }
                Signal::Jump { label } => {
                    if let Some(&new_idx) = labels.get(label) {
                        command_idx = new_idx;
                        continue;
                    } else {
                        return Err(format!("Runtime Error: Label '{}' not found", label));
                    }
                }
                Signal::SkipNext => {
                    command_idx += 2;
                }
                Signal::Continue => {
                    command_idx += 1
                }
            }
        }
        Ok(())
    }
    // Function returns Option<usize>. 
    // If Some(idx) was returned — GOTO worked and we need to jump to next index
    fn execute_single(
        &mut self, 
        stmt: &Statement<'a>) -> Result<Signal<'a>, String> {
        match stmt {
            Statement::Assign { name, value } => {
                let final_value = (value.evaluate(&self.env.map))?;
                self.env.set(name, final_value);
                Ok(Signal::Continue)
            }

            Statement::Input { name } => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)
                .map_err(|e| format!("Failed to read line: {}", e))?;
                let value: i64 = input.trim().parse()
                .map_err(|e| format!("Invalid integer input: {}",e))?;
                self.env.set(name, value);
                Ok(Signal::Continue)
            }

            Statement::PrintStr(text) => {
                println!("{}", text);
                Ok(Signal::Continue)
            }

            Statement::PrintVar(name) => {
                let val = self.env.get(name)?;
                println!("{val}");
                Ok(Signal::Continue)
            }

            Statement::GOTO { label } => {
                Ok(Signal::Jump { label })
            }
            
            
            Statement::IF { left_value, cmp, right_value} => {
                let lhs = left_value.evaluate(&self.env.map)?;
                let rhs = right_value.evaluate(&self.env.map)?;

                let condition = match cmp {
                    '=' => lhs == rhs,
                    '!' => lhs != rhs,
                    '<' => lhs < rhs, 
                    '>' => lhs > rhs,
                    _ => unreachable!(),
                };

                if condition {
                    Ok(Signal::Continue)
                } else {
                    Ok(Signal::SkipNext)
                }
            }

            Statement::Random { name, min, max } => {
                let mut rng = rand::thread_rng();
                let min_val = *min;
                let max_val = *max;

                let random_value: i64 = rng.gen_range(min_val..=max_val);
                
                // Записываем его в наше окружение, как обычный LET
                self.env.set(name, random_value);
                Ok(Signal::Continue)
            }

            Statement::Label { .. } => Ok(Signal::Continue),

            Statement::End => Ok(Signal::Exit),
        }
    }
}