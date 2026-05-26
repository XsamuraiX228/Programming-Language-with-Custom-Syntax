use std::collections::HashMap;
use super::lexer::Tokens;
use super::syntaxd::KeyWordType;
#[derive(Debug, PartialEq)]
pub enum OperationTree<'a> {
    Atom(i64),
    Variable(&'a str),
    Cons(char, Vec<OperationTree<'a>>)
}

impl<'a> OperationTree<'a> {
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

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command<'a> {
    Assign { name: &'a str, value: OperationTree<'a> },
    Input {name: &'a str},
    PrintStr(&'a str), // PRINT "HELLO"
    PrintVar(&'a str), // PRINT 10
    IF {left_value: OperationTree<'a>, cmp: char, right_value: OperationTree<'a>, body: Vec<Command<'a>>},
    Label {name: &'a str},
    GOTO {label: &'a str},
    Random {name:&'a str, min: i64, max: i64},
} 

pub struct Parser<'a> {
    tokens: Vec<Tokens<'a>>
}

impl<'a> Parser<'a> {

    pub fn new(mut tokens: Vec<Tokens<'a>>, ) -> Self {
        tokens.reverse();
        Self {tokens}
    }

    fn peek(&self) -> Option<&Tokens<'a>> {
        self.tokens.last()
    }

    fn next(&mut self) -> Option<Tokens<'a>> {
        self.tokens.pop()
    }

    fn get_name(&mut self) -> Result<&'a str, String> {
        let name = match self.next() {
            Some(Tokens::Ident(name)) => name,
            other => return Err(format!("Expected name of variable {:?}", other))
        };
        Ok(name)
    }

    fn get_num(&mut self) -> Result<i64, String> {
        let num = match self.next() {
            Some(Tokens::Number(num)) => num,
            other => return Err(format!("Expected name of variable {:?}", other))
        };
        Ok(num)
    }

    pub fn parse(&mut self) -> Result<Vec<Command<'a>>, String> {
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

    fn parse_command(&mut self) -> Result<Command<'a>, String> {
        let current_token = self.next().expect("Expected commad");

        match current_token {
            Tokens::KeyWord(KeyWordType::Let) => {
                let name = self.get_name()?;

                if self.next() != Some(Tokens::Equal) {
                    return Err(format!("Expected ="))
                }

                let value = self.expr_bp(0)?;
                Ok(Command::Assign { name, value })
            }

            Tokens::KeyWord(KeyWordType::Input) => {
                let name = self.get_name()?;

                Ok(Command::Input { name })
            }

            Tokens::KeyWord(KeyWordType::Print) => {
                // Смотрим, что идет после PRINT
                match self.next() {
                    Some(Tokens::Text(text)) => Ok(Command::PrintStr(text)),
                    Some(Tokens::Ident(name)) => Ok(Command::PrintVar(name)),
                    other => Err(format!("Expected string or variable after PRINT, found {:?}", other))
                }
            }

            Tokens::KeyWord(KeyWordType::If) => {
                let left_value = self.expr_bp(0)?;
                let op_token = self.next();
                let right_value = self.expr_bp(0)?;

                if self.next() != Some(Tokens::KeyWord(KeyWordType::Then)) {
                    return Err(format!("Expected block THEN"));
                }

                let body = self.parse_command()?;

                let cmp = match op_token {
                    Some(Tokens::DoubleEqual) => '=',
                    Some(Tokens::NonEqual) => '!',
                    Some(Tokens::Less) => '<',
                    Some(Tokens::Greater) => '>',
                    other => return Err(format!("Expected == or !=, got {:?}", other)),
                };
                Ok(Command::IF { left_value, cmp, right_value, body: vec![body] })
            }

            Tokens::Mark(name) => {
                Ok(Command::Label { name })
            }

            Tokens::KeyWord(KeyWordType::Goto) => {
                let label = self.get_name()?;
                Ok(Command::GOTO { label })
            }

            Tokens::KeyWord(KeyWordType::Random) => {
                let name = self.get_name()?;
                let min = self.get_num()?;
                let max = self.get_num()?;

                Ok (Command::Random { name, min, max })
            }

            other => Err(format!("Unexpected command token {:?}", other))
        }
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<OperationTree<'a>, String> {
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

            if let Ok((left_power, right_power)) = self.infix_bind_operator(op) {
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

    fn infix_bind_operator(&self, op: char) -> Result<(u8, u8), String> {
        match op {
            '+' | '-' => Ok((1,2)),
            '*' | '/' => Ok((3,4)),
            '^' => Ok((7,6)),
            _ => Err(format!("Wrong operator {}", op)) 
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