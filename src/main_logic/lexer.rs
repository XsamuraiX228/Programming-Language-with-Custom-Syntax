use super::syntaxd::{SyntaxDict, KeyWordType};

const VALID_OPERATORS: [char; 8] = ['+', '-', '*', '/', '^', '!', '(', ')'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tokens<'a> {   
    KeyWord(KeyWordType),        
    Ident(&'a str),
    Number(i64),  
    Equal,        
    Op(char),
    Newline,      
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    config: &'a SyntaxDict,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, config: &'a SyntaxDict) -> Self {
        Self {
            input: input,
            pos: 0,
            config
        }
    }

    fn next_token(&mut self) -> Option<Tokens<'a>> {
        let bytes = self.input.as_bytes();
        while self.pos < self.input.len() && bytes[self.pos] == b' ' {
            self.pos += 1
        }
        if self.pos > self.input.len() - 1 {
            return None;
        }
        let ch = bytes[self.pos] as char;

        match ch {
            '=' => {
                self.pos += 1;
                return Some(Tokens::Equal);
            }
            '\n' => {
                self.pos += 1;
                return Some(Tokens::Newline);
            }
            op if VALID_OPERATORS.contains(&op) => {
                self.pos += 1;
                return Some(Tokens::Op(ch));
            }
            '0'..'9' => {
                let start = self.pos;
                while self.pos < bytes.len() && (bytes[self.pos] as char).is_ascii_digit() {
                    self.pos += 1
                }
                let num_str = &self.input[start..self.pos];
                let number = num_str.parse::<i64>().unwrap();
                return Some(Tokens::Number(number));
            }
            _ => {
                let start = self.pos;
                while self.pos < bytes.len() {
                    let current_char = bytes[self.pos] as char;
                    if current_char.is_whitespace() 
                        || current_char == '=' 
                        || VALID_OPERATORS.contains(&current_char) 
                    {
                        break;
                    }
                    self.pos += 1;
                }
                let word_str = &self.input[start..self.pos];
                if let Some(kw_type) = self.config.keywords.get(word_str) {
                    return Some(Tokens::KeyWord(kw_type.clone()));
                }
                Some(Tokens::Ident(word_str))
            }
        }
    }
    pub fn tokenize(&mut self) -> Vec<Tokens<'a>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}
