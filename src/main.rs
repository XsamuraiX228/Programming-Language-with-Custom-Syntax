use basic_lexer::settings::{
    run
};

use basic_lexer::main_logic::syntaxd::Dictionaries;

fn main() {
    let program = "ПУСТЬ х = 10 * 2\nПЕЧАТЬ х";
    run(program, Dictionaries::Russian);

    let next_progamm = "LET x = 10 + 5 / 3 * 7\nPRINT x";

    run(next_progamm, Dictionaries::English);

    let emoji_programm = "✍️ y = 10 * 5 + 2 * 7\n🖨 y\n⌨️ x\n✍️ x = x + y\n🖨 x";
    run(emoji_programm, Dictionaries::Emoji);
}

#[cfg(test)]
mod tests {
    use basic_lexer::settings::run;
    use basic_lexer::main_logic::syntaxd::Dictionaries;
    use basic_lexer::main_logic::syntaxd::{SyntaxDict, Dictionaries as DictEnum};
    use basic_lexer::main_logic::lexer::Lexer;
    use basic_lexer::main_logic::parser::Parser;

    // =========================================================================
    // 1. ТЕСТЫ СЛОВАРЕЙ И ЛЕКСЕРА
    // =========================================================================

    #[test]
    fn test_emoji_lexing() {
        // Проверяем, что эмодзи-синтаксис корректно распознается лексером
        let config = SyntaxDict::choose_dict(DictEnum::Emoji);
        let mut lexer = Lexer::new("✍️ x = 5\n🖨 x", config);
        let tokens = lexer.tokenize();

        // Проверяем, что первый токен — это действительно KeyWord(Let)
        assert!(matches!(tokens[0], basic_lexer::main_logic::lexer::Tokens::KeyWord(_)));
        assert_eq!(tokens[1], basic_lexer::main_logic::lexer::Tokens::Ident("x".to_string()));
        assert_eq!(tokens[2], basic_lexer::main_logic::lexer::Tokens::Equal);
    }

    #[test]
    fn test_russian_lexing() {
        // Проверяем русский синтаксис
        let config = SyntaxDict::choose_dict(DictEnum::Russian);
        let mut lexer = Lexer::new("ПУСТЬ переменная = 42\nПЕЧАТЬ переменная", config);
        let tokens = lexer.tokenize();

        assert_eq!(tokens[1], basic_lexer::main_logic::lexer::Tokens::Ident("переменная".to_string()));
    }

    // =========================================================================
    // 2. ТЕСТЫ МАТЕМАТИКИ И ПРИОРИТЕТОВ (ПАРСЕР ПРАТТА)
    // =========================================================================

    #[test]
    fn test_pratt_parser_precedence() {
        let config = SyntaxDict::choose_dict(DictEnum::English);
        // Математика должна сгруппироваться как: (1 + ((2 * 3) / 3)) - 1
        let mut lexer = Lexer::new("LET x = 1 + 2 * 3 / 3 - 1", config);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let commands = parser.parse();

        // Проверяем, что распарсилась ровно одна команда
        assert_eq!(commands.len(), 1);
        
        // Можешь временно сделать отладочный вывод, чтобы увидеть структуру дерева:
        // println!("{衬:?}", commands[0]);
    }

    // =========================================================================
    // 3. ИНТЕГРАЦИОННЫЕ ТЕСТЫ (ПОЛНЫЙ ЦИКЛ ВЫПОЛНЕНИЯ)
    // =========================================================================

    #[test]
    fn test_full_execution_with_variables() {
        // Этот тест проверяет работу всей системы через твой фасад Settings
        // Программа считает математику, сохраняет в X, а потом использует X для вычисления Y
        let program = "✍️ x = 2 + 2 * 2\n✍️ y = x * 10";
        
        // Запускаем через наш конвейер!
        // Если где-то будет паника (например переменная не найдена или оператор сломался) — тест упадет.
        run(program, Dictionaries::Emoji);
    }

    #[test]
    #[should_panic(expected = "Runtime Error: переменная 'unknown' не найдена!")]
    fn test_interpreter_panic_on_undefined_variable() {
        // Тест-перестраховка: проверяем, что наш интерпретатор ПРАВИЛЬНО паникует,
        // если пользователь пытается использовать переменную, которой не существует.
        let program = "LET x = unknown + 5";
        run(program, Dictionaries::English);
    }
}