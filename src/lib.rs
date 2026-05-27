pub mod main_logic;
pub mod io;

pub mod settings {
    use crate::main_logic::syntaxd::{Dictionaries, SyntaxDict};
    use crate::main_logic::lexer::Lexer;
    use crate::main_logic::parser::Parser;
    use crate::main_logic::interpreter::Interpreter;
    use crate::io::scanner::scan;
    use std::path::PathBuf;

    pub fn create_lexer<'a>(input: &'a str, config: &'a SyntaxDict) -> Lexer<'a> {
        Lexer::new(input, config)
    }

    pub fn scan_code(dir: &str) -> Result<Vec<PathBuf>, String> {
        scan(dir).map_err(|e| format!("Coudln't read dir {dir}: {e}"))
    }

    pub fn load_code(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    pub fn run<'a>(program: &'a str, config: Dictionaries) {
        // 1. Готовим конфигурацию синтаксиса
        let main_config = SyntaxDict::choose_dict(config);
        // 2. Создаем лексер (передаем config по ссылке &config) и получаем токены
        let mut lexer = create_lexer(program, &main_config);
        let tokens = lexer.tokenize();
        
        // 3. Передаем токены в парсер и строим дерево команд (AST)
        let mut parser = Parser::new(tokens);
        let commands = parser.parse().expect("Expected Vec<Command>");
        //dbg!(&commands);
        
        // 4. Запускаем интерпретатор на выполнение кода
        let mut interpreter = Interpreter::new();
        interpreter.get_marks(&commands);
    }
}