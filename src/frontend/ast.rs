use std::collections::HashMap;
use std::cell::Cell;
use super::token::OpType;
#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Atom(i64),
    Variable(&'a str),
    Cons(OpType, Vec<Expression<'a>>)
}

impl<'a> Expression<'a> {
    pub fn evaluate(&self, env: &HashMap<&'a str, i64>) -> Result<i64, String> {
        match self {
            Expression::Atom(n) => Ok(*n),
            Expression::Variable(name) => {
                env.get(*name)
                    .cloned()
                    .ok_or_else(|| format!("Variable '{}' not found", name))
            }
            Expression::Cons(op, args) => {
                match op {
                    OpType::Plus => {
                        if args.len() == 1 {
                            Ok(args[0].evaluate(env)?) 
                        } else {
                            Ok(args[0].evaluate(env)? + args[1].evaluate(env)?)
                        }
                    }
                    OpType::Minus => {
                        if args.len() == 1 {
                            Ok(-args[0].evaluate(env)?)  
                        } else {
                            Ok(args[0].evaluate(env)? - args[1].evaluate(env)?)
                        }
                    }
                    OpType::Multiply => {
                        Ok(args[0].evaluate(env)? * args[1].evaluate(env)?)
                    }
                    OpType::Divide => {
                        let rhs = args[1].evaluate(env)?;
                        if rhs == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(args[0].evaluate(env)? / rhs)
                    }
                    OpType::Mod => {
                        Ok(args[0].evaluate(env)? % args[1].evaluate(env)?)
                    }
                    OpType::Power => {
                        let base = args[0].evaluate(env)?;
                        let exp = args[1].evaluate(env)?;
                        Ok(base.pow(exp as u32))
                    }
                    OpType::Factorial => {
                        let val = args[0].evaluate(env)?;
                        Ok(factorial(val))
                    }
                    OpType::LParen | OpType::RParen => {
                        Err("Parentheses should not appear in evaluation".to_string())
                    }
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
    If {
        left_value: Expression<'a>, 
        cmp: &'a str, 
        right_value: Expression<'a>, 
        then_block: Vec<Statement<'a>>, 
        else_block: Vec<Statement<'a>>,
    }, // If statement
    While {left_value: Expression<'a>, cmp: &'a str, right_value: Expression<'a>, end_idx: Cell<usize>},
    WEnd {start_idx: usize},
    Label {name: &'a str}, // Mark to control the position where the GOTO will jump
    Goto {label: &'a str}, // Jump to mark in code
    Random {name:&'a str, min: i64, max: i64}, // Set random value to variable
    End, // Stop the program
} 
