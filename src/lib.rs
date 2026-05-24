pub mod main_logic;

pub mod settings {
    use crate::main_logic::syntaxd::{Dictionaries, SyntaxDict};
    use crate::main_logic::lexer::Lexer;
    use crate::main_logic::parser::Parser;
    use crate::main_logic::interpreter::Interpreter;

    pub fn set_dict(dicts: Dictionaries) -> SyntaxDict {
        SyntaxDict::choose_dict(dicts) 
    }

    pub fn create_lexer(input: &str, config: SyntaxDict) -> Lexer {
        Lexer::new(input, config)
    }

    pub fn run(input: &str, dicts: Dictionaries) {
        // 1. Готовим конфигурацию синтаксиса
        let config = set_dict(dicts);
        
        // 2. Создаем лексер и бьем строку на токены
        let mut lexer = create_lexer(input, config);
        let tokens = lexer.tokenize();
        
        // 3. Передаем токены в парсер и строим дерево команд (AST)
        let mut parser = Parser::new(tokens);
        let commands = parser.parse();
        
        // 4. Запускаем интерпретатор на выполнение кода
        let mut interpreter = Interpreter::new();
        interpreter.execute(commands);
    }
}