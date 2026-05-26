pub mod main_logic;

pub mod settings {
    use crate::main_logic::syntaxd::{Dictionaries, SyntaxDict};
    use crate::main_logic::lexer::Lexer;
    use crate::main_logic::parser::Parser;
    use crate::main_logic::interpreter::Interpreter;

    pub fn set_dict(dicts: Dictionaries) -> SyntaxDict {
        SyntaxDict::choose_dict(dicts) 
    }

    pub fn create_lexer<'a>(input: &'a str, config: &'a SyntaxDict) -> Lexer<'a> {
        Lexer::new(input, config)
    }

    pub fn run<'a>(program: &'a str, dictionary_type: Dictionaries) {
        // 1. Готовим конфигурацию синтаксиса
        let config = set_dict(dictionary_type);
        
        // 2. Создаем лексер (передаем config по ссылке &config) и получаем токены
        let mut lexer = create_lexer(program, &config);
        let tokens = lexer.tokenize();
        dbg!(&tokens);
        
        // 3. Передаем токены в парсер и строим дерево команд (AST)
        let mut parser = Parser::new(tokens);
        let commands = parser.parse().expect("Expected Vec<Command>");
        //dbg!(&commands);
        
        // 4. Запускаем интерпретатор на выполнение кода
        let mut interpreter = Interpreter::new();
        interpreter.get_marks(&commands);
    }
}