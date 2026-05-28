use crate::frontend::ast::Statement;
use crate::dialect::KeyWordType;
use super::token::Token;
use super::ast::Expression;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {

    pub fn new(mut tokens: Vec<Token<'a>>, ) -> Self {
        tokens.reverse();
        Self {tokens}
    }

    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.last()
    }

    fn next(&mut self) -> Option<Token<'a>> {
        self.tokens.pop()
    }

    fn get_name(&mut self) -> Result<&'a str, String> {
        let name = match self.next() {
            Some(Token::Ident(name)) => name,
            other => return Err(format!("Expected name of variable {:?}", other))
        };
        Ok(name)
    }

    fn get_num(&mut self) -> Result<i64, String> {
        let num = match self.next() {
            Some(Token::Number(num)) => num,
            other => return Err(format!("Expected name of variable {:?}", other))
        };
        Ok(num)
    }

    pub fn parse(&mut self) -> Result<Vec<Statement<'a>>, String> {
        let mut commands = Vec::new();

        while let Some(token) = self.peek() {
            // Skip blank pages
            if token == &Token::Newline {
                self.next();
                continue;
            }

            // Parse line
            let cmd = self.parse_command()?;
            commands.push(cmd);

            if let Some(Token::Newline) = self.peek() {
                self.next();
            }
        }

        Ok(commands)
    }

    fn parse_command(&mut self) -> Result<Statement<'a>, String> {
        let current_token = self.next().expect("Expected commad");

        match current_token {
            Token::KeyWord(KeyWordType::Let) => self.parse_let(),
            Token::KeyWord(KeyWordType::Print) => self.parse_print(),
            Token::KeyWord(KeyWordType::If) => self.parse_if(),
            Token::KeyWord(KeyWordType::Random) => self.parse_random(),

            Token::Mark(name) => Ok(Statement::Label { name }),
            Token::KeyWord(KeyWordType::End) => Ok(Statement::End),
            Token::KeyWord(KeyWordType::Input) => {
                let name = self.get_name()?;
                Ok(Statement::Input { name })
            }
            Token::KeyWord(KeyWordType::Goto) => {
                let label = self.get_name()?;
                Ok(Statement::GOTO { label })
            }
            other => Err(format!("Unexpected command token {:?}", other))
        }
    }
    
    fn parse_let(&mut self) -> Result<Statement<'a>, String> {
        // get variable name
        let name = self.get_name()?;

        // check if = exist
        if self.next() != Some(Token::Equal) {
            return Err(format!("Expected ="))
        }

        // get the value, which will be stored in hashmap
        let value = self.expr_bp(0)?;
        Ok(Statement::Assign { name, value })
    }

    fn parse_print(&mut self) -> Result<Statement<'a>, String> {
        // match if we need to print number or string
        match self.next() {
            // String 
            Some(Token::Text(text)) => Ok(Statement::PrintStr(text)),
            // Number or variable which store number
            Some(Token::Ident(name)) => Ok(Statement::PrintVar(name)),
            other => Err(format!("Expected string or variable after PRINT, found {:?}", other))
        }
    }

    fn parse_if(&mut self) -> Result<Statement<'a>, String> {
        // get left expression
        let left_value = self.expr_bp(0)?;

        // get operator (==, !=, <, >)
        let op_token = self.next();
        let cmp = match op_token {
            Some(Token::DoubleEqual) => '=',
            Some(Token::NonEqual) => '!',
            Some(Token::Less) => '<',
            Some(Token::Greater) => '>',
            other => return Err(format!("Expected == or !=, got {:?}", other)),
        };

        // get right expression
        let right_value = self.expr_bp(0)?;

        // check if keyword THEN exist
        if self.next() != Some(Token::KeyWord(KeyWordType::Then)) {
                return Err(format!("Expected block THEN"));
        }

        // get the command after THEN to execute it, depending on the result of IF
        let body = self.parse_command()?;
        Ok(Statement::IF { 
            left_value, 
            cmp, 
            right_value, 
            body: vec![body] 
        })
    }

    fn parse_random(&mut self) -> Result<Statement<'a>, String> {
        // get name of varibale
        let name = self.get_name()?;
        
        // get min and max to set a borders
        let min = self.get_num()?;
        let max = self.get_num()?;

        Ok (Statement::Random { name, min, max })
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<Expression<'a>, String> {
        let mut lhs = match self.next() {
            Some(Token::Number(num)) => Expression::Atom(num),
            Some(Token::Op('(')) => {
                let lhs = self.expr_bp( 0)?;
                if self.next() != Some(Token::Op(')')) {
                    return Err("Syntax Error: Expected matching ')'".to_string());
                }
                lhs
            }
            Some(Token::Op(op)) => {
                let ((), r_bp) = self.prefix_bind_opartor(op)?;
                let rhs = self.expr_bp( r_bp)?;
                Expression::Cons(op, vec![rhs])
            }
            Some(Token::Ident(name)) => Expression::Variable(name),
            t => return Err(format!("Expected number, variable or prefix operator, but found {:?}", t)),
        }; 

        loop {
            let op = match self.peek() {
                Some(Token::Op(op)) => *op,
                _ => break
            };

            if let Ok((left_power, ())) = self.postfix_bind_operator(op) {
                if left_power < min_bp {
                    break;
                }
                self.next();

                lhs = Expression::Cons(op, vec![lhs]);
                continue;
            }

            if let Ok((left_power, right_power)) = self.infix_bind_operator(op) {
                if left_power < min_bp {
                    break;
                }
                self.next();
                let rhs = self.expr_bp(right_power)?;
                lhs = Expression::Cons(op, vec![lhs, rhs]);
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
