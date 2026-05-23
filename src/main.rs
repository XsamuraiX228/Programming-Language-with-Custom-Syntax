#[derive(Debug, PartialEq, Clone)]
enum Tokens {
    Let,          // Ключевое слово "LET"
    Print,        // Ключевое слово "PRINT"
    Ident(String),// Имя переменной, например "x" или "my_var"
    Number(i32),  // Число, например "10"
    Equal,        // Знак равенства "="
    Newline,      // Конец строки (в BASIC это важно)
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }
    fn next_token(&mut self) -> Option<Tokens> {
        while self.pos < self.input.len() && self.input[self.pos] != ' ' {
            self.pos += 1
        }
        if self.pos > self.input.len() {
            return None;
        }

        let ch = self.input[self.pos];

        if ch == '=' {
            self.pos += 1;
            return Some(Tokens::Equal);
        }

        if ch == '\n' {
            self.pos += 1;
            return Some(Tokens::Newline);
        }

        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            while self.pos < self.input.len() && ch.is_ascii_digit() {
                num_str.push(self.input[self.pos]);
                self.pos += 1
            }
            let number = num_str.parse::<i32>().unwrap();
            return Some(Tokens::Number(number));
        }

        if ch.is_ascii_alphabetic() {
            let mut word_str = String::new();
            while self.pos < self.input.len() && ch.is_ascii_alphabetic() {
                word_str.push(self.input[self.pos]);
                self.pos += 1
            }
            match word_str.as_str() {
                "LET" => return Some(Tokens::Let),
                "PRINT" => return  Some(Tokens::Print),
                _ => return Some(Tokens::Ident(word_str.to_string())),
            }
        }

        panic!("Syntax Error, EBALN KONCHENY")
    }
}


fn main() {
    println!("Hello, world!");
}
