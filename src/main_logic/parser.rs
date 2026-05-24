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
    pub fn evaluate(&self, env: &HashMap<String, i32>) -> i32 {
        match self {
            Self::Atom(n) => {
                *n
            },
            Self::Variable(name) => {
                *env.get(name).unwrap_or_else(|| {
                    panic!("Runtime Error: переменная '{}' не найдена!", name);
                })
            }
            Self::Cons(op,args ) => {
                let lhs = args[0].evaluate(env);
                let rhs = args[1].evaluate(env);

                match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    _ => panic!("Unknow operator {}", op)
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

    pub fn parse(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();

        while self.peek().is_some() {
            if self.peek() == Some(&Tokens::Newline) {
                self.next();
                continue;
            }

            let cmd = self.parse_command();
            commands.push(cmd);

            if let Some(Tokens::Newline) = self.peek() {
                self.next();
            } else if self.peek().is_some() {
                panic!("Expected next line {:?}", self.peek())
            }
        }

        commands
    }

    fn parse_command(&mut self) -> Command {
        let current_token = self.next().expect("Expected commad");

        match current_token {
            Tokens::KeyWord(KeyWordType::Let) => {
                let name = match self.next() {
                    Some(Tokens::Ident(name)) => name,
                    other => panic!("Expected name of variable {:?}", other)
                };

                if self.next() != Some(Tokens::Equal) {
                    panic!("Expected =");
                }

                let value = self.expr_bp(0);

                Command::Assign { name, value }
            }

            Tokens::KeyWord(KeyWordType::Input) => {
                let name = match self.next() {
                    Some(Tokens::Ident(name)) => name,
                    other => panic!("Expected name of variable {:?}", other)
                };

                Command::Input { name }
            }

            Tokens::KeyWord(KeyWordType::Print) => {
                let name = match self.next() {
                    Some(Tokens::Ident(name)) => name,
                    other => panic!("Expected name of variable {:?}", other)
                };
                Command::Print { name }
            }

            other => panic!("Unexpected command {:?}", other)
        }
    }

    fn expr_bp(&mut self, min_bp: u8) -> OperationTree {
        let mut lhs = match self.next() {
            Some(Tokens::Number(num)) => OperationTree::Atom(num),
            Some(Tokens::Ident(name)) => OperationTree::Variable(name),
            t => panic!("Expected number or variable, but {:?} were given", t),
        }; 

        loop {
            let op = match self.peek() {
                Some(Tokens::Op(op)) => *op,
                _ => break
            };
            let (left_power, right_power) = self.infixing_operator(op);
            if left_power < min_bp {
                break;
            }

            self.next();
            let rhs = self.expr_bp(right_power);
            lhs = OperationTree::Cons(op, vec![lhs, rhs])
        }

        lhs
    }
    fn infixing_operator(&self, op: char) -> (u8, u8) {
        match op {
            '+' | '-' => (1,2),
            '*' | '/' => (3,4),
            _ => panic!("bad op: {:?}", op) 
        }
    }
}