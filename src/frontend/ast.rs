use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Atom(i64),
    Variable(&'a str),
    Cons(char, Vec<Expression<'a>>)
}

impl<'a> Expression<'a> {
    pub fn evaluate(&self, env: &HashMap<&'a str, i64>) -> Result<i64, String> {
        match self {
            Self::Atom(n) => {
                Ok(*n)
            },
            Self::Variable(name) => {
                env.get(*name)
                    .cloned()
                    .ok_or_else(|| format!("Runtime Error: variable '{}' not found!", name))
            }
            Self::Cons(op,args ) => {
                match args.len() {
                    1 => {
                        let rhs = args[0].evaluate(env)?;
                        match op {
                            '+' => Ok(rhs),
                            '-' => Ok(-rhs),
                            '!' => Ok(factorial(rhs)),
                            t => Err(format!("Wrong operator {t} ")),
                        }
                    }

                    2 => {
                        let lhs = args[0].evaluate(env)?;
                        let rhs = args[1].evaluate(env)?;

                        match op {
                            '+' => Ok(lhs + rhs),
                            '-' => Ok(lhs - rhs),
                            '*' => Ok(lhs * rhs),
                            '/' => {
                                if rhs == 0 {
                                    return Err("Runtime Error: Division by zero".to_string());
                                }
                                Ok(lhs / rhs)
                            }
                            '^' => Ok(lhs.pow(rhs as u32)),
                            _ => return Err(format!("Unknow operator {}", op))
                        }
                    }
                    _ => Err(format!("Wrong letght of args"))
                }
                
            }
        }
    }
}

fn factorial(mut x: i64) -> i64 {
    let mut sum = 1;
    while x > 1 {
        sum *= x;
        x -= 1;
    }
    sum
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement<'a> {
    Assign { name: &'a str, value: Expression<'a> }, // Assign the value to variable
    Input {name: &'a str}, // Input the value
    PrintStr(&'a str), // Print strings
    PrintVar(&'a str), // Get value from variables and print it
    IF {left_value: Expression<'a>, cmp: char, right_value: Expression<'a>}, // If statement
    Label {name: &'a str}, // Mark to control the position where the GOTO will jump
    GOTO {label: &'a str}, // Jump to mark in code
    Random {name:&'a str, min: i64, max: i64}, // Set random value to variable
    End, // Stop the program
} 