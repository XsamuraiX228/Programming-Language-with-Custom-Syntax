use super::syntaxd::{SyntaxDict, KeyWordType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tokens {   
    KeyWord(KeyWordType),        
    Ident(String),
    Number(i32),  
    Equal,        
    Op(char),
    Newline,      
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    config: SyntaxDict,
}

impl Lexer {
    pub fn new(input: &str, config: SyntaxDict) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            config
        }
    }

    fn next_token(&mut self) -> Option<Tokens> {
        while self.pos < self.input.len() && self.input[self.pos] == ' ' {
            self.pos += 1
        }
        if self.pos > self.input.len() - 1 {
            return None;
        }

        let ch = self.input[self.pos];

        if ch == '=' {
            self.pos += 1;
            return Some(Tokens::Equal);
        }

        if ch == '+' || ch == '*' || ch == '-' || ch == '/' {
            self.pos += 1;
            return Some(Tokens::Op(ch));
        }

        if ch == '\n' {
            self.pos += 1;
            return Some(Tokens::Newline);
        }
        // LET var = 5555 PRINT INPUT
        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                num_str.push(self.input[self.pos]);
                self.pos += 1
            }
            let number = num_str.parse::<i32>().unwrap();
            return Some(Tokens::Number(number));
        }

        if !ch.is_whitespace() && ch != '=' && !ch.is_ascii_digit() {
            let mut word_str = String::new();
            while self.pos < self.input.len() && !self.input[self.pos].is_whitespace() && self.input[self.pos] != '='  {
                word_str.push(self.input[self.pos]);
                self.pos += 1
            }
            if let Some(kw_type) = self.config.keywords.get(&word_str) {
                return Some(Tokens::KeyWord(kw_type.clone()));
            }

            return Some(Tokens::Ident(word_str));
        }

        panic!("Syntax Error, EBLAN KONCHENY")
    }

    pub fn tokenize(&mut self) -> Vec<Tokens> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}