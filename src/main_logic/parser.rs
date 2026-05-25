use std::collections::HashMap;
use super::lexer::Tokens;
use super::syntaxd::KeyWordType;
#[derive(Debug)]
pub enum OperationTree {
    Atom(i32),
    Variable(String),
    Cons(char, Vec<OperationTree>)
}

impl OperationTree {
    pub fn evaluate(&self, env: &HashMap<String, i32>) -> Result<i32, String> {
        match self {
            Self::Atom(n) => {
                Ok(*n)
            },
            Self::Variable(name) => {
                env.get(name)
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
                            '^' => Ok(power(lhs, rhs)),
                            _ => return Err(format!("Unknow operator {}", op))
                        }
                    }
                    _ => Err(format!("Wrong letght of args"))
                }
                
            }
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Assign { name: String, value: OperationTree },
    Input {name: String},
    Print {name: String},
} 

pub struct Parser {
    tokens: Vec<Tokens>
}

impl Parser {

    pub fn new(mut tokens: Vec<Tokens>, ) -> Self {
        tokens.reverse();
        Self {tokens}
    }

    fn peek(&self) -> Option<&Tokens> {
        self.tokens.last()
    }

    fn next(&mut self) -> Option<Tokens> {
        self.tokens.pop()
    }

    pub fn parse(&mut self) -> Result<Vec<Command>, String> {
        let mut commands = Vec::new();

        while self.peek().is_some() {
            if self.peek() == Some(&Tokens::Newline) {
                self.next();
                continue;
            }

            let cmd = self.parse_command()?;
            commands.push(cmd);

            if let Some(Tokens::Newline) = self.peek() {
                self.next();
            } else if self.peek().is_some() {
                panic!("Expected next line {:?}", self.peek())
            }
        }

        Ok(commands)
    }

    fn parse_command(&mut self) -> Result<Command, String> {
        let current_token = self.next().expect("Expected commad");

        match current_token {
            Tokens::KeyWord(KeyWordType::Let) => {
                let name = match self.next() {
                    Some(Tokens::Ident(name)) => name,
                    other => return Err(format!("Expected name of variable {:?}", other))
                };

                if self.next() != Some(Tokens::Equal) {
                    return Err(format!("Expected ="))
                }

                let value = self.expr_bp(0)?;
                Ok(Command::Assign { name, value })
            }

            Tokens::KeyWord(KeyWordType::Input) => {
                let name = match self.next() {
                    Some(Tokens::Ident(name)) => name,
                    other => return Err(format!("Expected name of variable {:?}", other))
                };

                Ok(Command::Input { name })
            }

            Tokens::KeyWord(KeyWordType::Print) => {
                let name = match self.next() {
                    Some(Tokens::Ident(name)) => name,
                    other => return Err(format!("Expected name of variable {:?}", other))
                };
                Ok(Command::Print { name })
            }

            other => Err(format!("Unexpected command token {:?}", other))
        }
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<OperationTree, String> {
        let mut lhs = match self.next() {
            Some(Tokens::Number(num)) => OperationTree::Atom(num),
            Some(Tokens::Op('(')) => {
                let lhs = self.expr_bp( 0)?;
                if self.next() != Some(Tokens::Op(')')) {
                    return Err("Syntax Error: Expected matching ')'".to_string());
                }
                lhs
            }
            Some(Tokens::Op(op)) => {
                let ((), r_bp) = self.prefix_bind_opartor(op)?;
                let rhs = self.expr_bp( r_bp)?;
                OperationTree::Cons(op, vec![rhs])
            }
            Some(Tokens::Ident(name)) => OperationTree::Variable(name),
            t => return Err(format!("Expected number, variable or prefix operator, but found {:?}", t)),
        }; 

        loop {
            let op = match self.peek() {
                Some(Tokens::Op(op)) => *op,
                _ => break
            };

            if let Ok((left_power, ())) = self.postfix_bind_operator(op) {
                if left_power < min_bp {
                    break;
                }
                self.next();

                lhs = OperationTree::Cons(op, vec![lhs]);
                continue;
            }

            if let Ok((left_power, right_power)) = self.infixing_operator(op) {
                if left_power < min_bp {
                    break;
                }

                self.next();
                let rhs = self.expr_bp(right_power)?;
                lhs = OperationTree::Cons(op, vec![lhs, rhs]);
                continue;
            }
            break;
        }

        Ok(lhs)
    }
    fn prefix_bind_opartor(&self, op: char) -> Result<((), u8), String> {
        match op {
            '+' | '-' => Ok(((), 5)),
            t => Err(format!("Wrong operator {:?}", t)),
        }
    }

    fn postfix_bind_operator(&self, op: char) ->Result<(u8, ()), String> {
        match op {
            '!' => Ok((8, ())),
            t => Err(format!("Wrong operator {:?}", t)),
        }
    }

    fn infixing_operator(&self, op: char) -> Result<(u8, u8), String> {
        match op {
            '+' | '-' => Ok((1,2)),
            '*' | '/' => Ok((3,4)),
            '^' => Ok((7,6)),
            _ => Err(format!("Wrong operator {}", op)) 
        }
    }
}

// Additional math functions
fn power(base: i32, exponent: i32) -> i32 {
    base.pow(exponent as u32)
}

fn factorial(mut x: i32) -> i32 {
    let mut sum = 1;
    while x > 1 {
        sum *= x;
        x -= 1;
    }
    sum
}