use std::collections::HashMap;


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

        if ch == '\n' {
            self.pos += 1;
            return Some(Tokens::Newline);
        }

        if ch.is_ascii_digit() {
            let mut num_str = String::new();
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                num_str.push(self.input[self.pos]);
                self.pos += 1
            }
            let number = num_str.parse::<i32>().unwrap();
            return Some(Tokens::Number(number));
        }

        if ch.is_ascii_alphabetic() {
            let mut word_str = String::new();
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_alphabetic() {
                word_str.push(self.input[self.pos]);
                self.pos += 1
            }
            match word_str.as_str() {
                "LET" => return Some(Tokens::Let),
                "PRINT" => return  Some(Tokens::Print),
                _ => return Some(Tokens::Ident(word_str.to_string())),
            }
        }

        panic!("Syntax Error, EBLAN KONCHENY")
    }

    fn tokenisize(&mut self) -> Vec<Tokens> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Command {
    Assign { name: String, value: i32 },
    Print {name: String},
}

struct Parser {
    tokens: Vec<Tokens>,
    pos: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Tokens>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();

        while self.pos < self.tokens.len() {
            if self.tokens[self.pos] == Tokens::Newline {
                self.pos += 1;
                continue;
            }

            let cmd = self.parse_command();
            commands.push(cmd);

            if self.pos < self.tokens.len() {
                if let Tokens::Newline = self.tokens[self.pos] {
                    self.pos += 1
                } else {
                    panic!("Expected next line at {:?}", self.tokens[self.pos])
                }
            }
        }

        commands
    }

    fn parse_command(&mut self) -> Command {
        let current_token = &self.tokens[self.pos];

        match current_token {
            Tokens::Let => {

                // LET x = 5

                self.pos += 1; // Current line starts with LET

                // 1. Get the name of variable
                let name = match &self.tokens[self.pos] {
                    Tokens::Ident(name) => name.clone(), // The name found (e.g x)
                    other => panic!("After LET expected =, but {:?} were given", other),
                };
                self.pos += 1; // Next

                // 2. Check if the line has "="
                if self.tokens[self.pos] != Tokens::Equal {
                    panic!("Expected =, but {:?} were given", self.tokens[self.pos]);
                }
                self.pos += 1; // Next

                // 3. Get value of variable
                let value = match &self.tokens[self.pos] {
                    Tokens::Number(val) => *val, // Нашли! Сохраняем число (например, 42)
                    other => panic!("Expected number,but {:?} were given", other),
                };
                self.pos += 1; // Next

                // We created a command
                // We return this commnad to execute it
                Command::Assign { name, value }
            }

            Tokens::Print => {

                // PRINT x

                self.pos += 1;
                
                let name = match &self.tokens[self.pos] {
                    Tokens::Ident(name) => name.clone(),
                    other => panic!("After LET expected =, but {:?} were given", other),
                };
                self.pos += 1;

                Command::Print { name }
            }

            other => panic!("Unexpected command {:?}", other)
        }
    }
}


struct InterPrenter {
    env: HashMap<String, i32>
}

impl InterPrenter {
    fn new() -> Self {
        Self { env: HashMap::new()}
    }

    fn execute(&mut self, commads: Vec<Command>) {
        for cmd in commads {
            match cmd {
                Command::Assign { name, value } => {
                    self.env.insert(name, value);
                }

                Command::Print { name } => {
                    if let Some(val) = self.env.get(&name) {
                        println!("{}", val)
                    } else {
                        panic!("Runtime Error: variable '{}' is not defined", name);
                    }
                }
            }
        }
    }


}


fn main() {
    // main BASIC
    let source_code = "LET x = 10\nPRINT z"; // z не существует!

    println!("--- Запуск программы ---");

    // 1. Lexer
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenisize();

    // 2. Parser
    let mut parser = Parser::new(tokens);
    let commands = parser.parse();

    // 3. Launch InterPrenter
    let mut interpreter = InterPrenter::new();
    interpreter.execute(commands);
}