use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Clone)]
enum KeyWordType {
    Let,
    Print,
    Input,
}

#[derive(Debug, PartialEq, Clone)]
enum Tokens {   
    KeyWord(KeyWordType),        
    Ident(String),
    Number(i32),  
    Equal,        
    Newline,      
}

struct SyntaxDict {
    keywords: HashMap<String, KeyWordType>
}

impl SyntaxDict {
    fn default_basic() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("LET".to_string(), KeyWordType::Let);
        keywords.insert("PRINT".to_string(), KeyWordType::Print);
        keywords.insert("INPUT".to_string(), KeyWordType::Input);
        Self { keywords }
    }

    // Кастомный русский синтаксис!
    fn russian_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("ПУСТЬ".to_string(), KeyWordType::Let);
        keywords.insert("ПЕЧАТЬ".to_string(), KeyWordType::Print);
        keywords.insert("ВВОД".to_string(), KeyWordType::Input);
        Self { keywords }
    }

    fn emoji_style() -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("✍️".to_string(), KeyWordType::Let);
        keywords.insert("🖨".to_string(), KeyWordType::Print);
        keywords.insert("⌨️".to_string(), KeyWordType::Input);
        Self { keywords }
    }
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
    config: SyntaxDict,
}

impl Lexer {
    fn new(input: &str, config: SyntaxDict) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            config
        }
    }

    // "                   LET"
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
    Input {name: String},
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
            Tokens::KeyWord(KeyWordType::Let) => {

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

            Tokens::KeyWord(KeyWordType::Input) => {
                self.pos += 1;

                //INPUT
                let name = match &self.tokens[self.pos] {
                    Tokens::Ident(name) => name.clone(), // The name found (e.g x)
                    other => panic!("After LET expected =, but {:?} were given", other),
                };

                self.pos += 1;

                Command::Input { name }
            }

            Tokens::KeyWord(KeyWordType::Print) => {

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

                Command::Input { name } => {
                    let mut input = String::new();

                    io::stdin().read_line(&mut input).unwrap();
                    let value: i32 = input.trim().parse().expect("Expected number, but string were give");
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
    // --- ВАРИАНТ 1: Стандартный BASIC ---
    let english_code = "INPUT x\nPRINT x";
    let config_eng = SyntaxDict::default_basic();
    
    let mut lexer_eng = Lexer::new(english_code, config_eng);
    let tokens_eng = lexer_eng.tokenisize();
    
    let mut parser_eng = Parser::new(tokens_eng);
    let ast_eng = parser_eng.parse();
    println!("AST из английского кода: {:?}", ast_eng);

    let mut interprenter = InterPrenter::new();
    interprenter.execute(ast_eng);


    // --- ВАРИАНТ 2: Русский BASIC! ---
    let russian_code = "ВВОД переменная\nПЕЧАТЬ переменная";
    let config_rus = SyntaxDict::russian_style(); // Подгружаем русские ключевые слова
    
    let mut lexer_rus = Lexer::new(russian_code, config_rus);
    let tokens_rus = lexer_rus.tokenisize();
    
    let mut parser_rus = Parser::new(tokens_rus);
    let ast_rus = parser_rus.parse();
    println!("AST из русского кода: {:?}", ast_rus);

    interprenter.execute(ast_rus);

    let emoji_code = "✍️ x = 777\n🖨 x\n⌨️ кастомный_ввод\n🖨 кастомный_ввод";

    let config = SyntaxDict::emoji_style();
    
    // Лексер
    let mut lexer = Lexer::new(emoji_code, config);
    let tokens = lexer.tokenisize();
    
    // Парсер
    let mut parser = Parser::new(tokens);
    let ast_emoji = parser.parse();

    println!("--- Исходный код на Эмодзи ---");
    println!("{}", emoji_code);
    println!("AST из эмодзи кода: {:?}", ast_emoji);

    
    // Интерпретатор
    println!("--- Выполнение программы ---");
    interprenter.execute(ast_emoji);
}