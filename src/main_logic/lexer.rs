use super::syntaxd::{SyntaxDict, KeyWordType};

const VALID_OPERATORS: [char; 7] = ['+', '-', '*', '/', '^', '(', ')'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tokens<'a> {   
    KeyWord(KeyWordType), // Let, Input, Print, If, Then, Random
    Ident(&'a str), // simple string
    Text(&'a str),
    Number(i64),  // i64 number
    Equal, // =
    DoubleEqual, // ==
    NonEqual, // !=
    Op(char), // ['+', '-', '*', '/', '^', '(', ')']
    Newline, // \n
    Mark(&'a str), // e.g :loop 
    Less,
    Greater,
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
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    return Some(Tokens::DoubleEqual)
                }
                self.pos += 1;
                return Some(Tokens::Equal);
            }
            '!' => {
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    return Some(Tokens::NonEqual)
                }
                self.pos += 1;
                return Some(Tokens::Op('!'));
            }
            '<' => {
                self.pos += 1;
                return Some(Tokens::Less);
            }
            '>' => {
                self.pos += 1;
                return Some(Tokens::Greater); 
            }
            '\n' => {
                self.pos += 1;
                return Some(Tokens::Newline);
            }
            op if VALID_OPERATORS.contains(&op) => {
                self.pos += 1;
                return Some(Tokens::Op(ch));
            }
            '"' => {
                self.pos += 1; 
                let start = self.pos;
                
                while self.pos < bytes.len() && bytes[self.pos] != b'"' {
                    self.pos += 1;
                }
                
                let text_str = &self.input[start..self.pos];
                self.pos += 1; 
                Some(Tokens::Text(text_str))
            }
            ':' => {
                self.pos += 1;
                let start = self.pos;
                while self.pos < bytes.len() {
                    let current_char = bytes[self.pos] as char;
                    if current_char.is_whitespace() 
                        || current_char == '='
                        || current_char == '!'
                        || VALID_OPERATORS.contains(&current_char) 
                    {
                        break;
                    }
                    self.pos += 1;
                }
                Some(Tokens::Mark(&self.input[start..self.pos]))
            }
            '0'..='9' => {
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
                        || current_char == '!'
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
