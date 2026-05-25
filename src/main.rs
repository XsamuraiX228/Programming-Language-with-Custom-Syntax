use basic_lexer::settings::{
    run
};

use basic_lexer::main_logic::syntaxd::Dictionaries;

fn main() {
    // ТЕСТ 1: Классический макрос (English)
    // Проверяем: последовательное выполнение, создание переменных,
    // использование старых переменных в новых формулах и перезапись (b = b + c).
    let macro_program = "LET a = 5\nLET b = a ^ 2 + 3\nLET c = (b - a) * 2!\nPRINT a\nPRINT b\nPRINT c\nLET b = b + c\nPRINT b";
    
    println!("--- ЗАПУСК ТЕСТА 1 (English) ---");
    run(macro_program, Dictionaries::English);


    // ТЕСТ 2: Эмодзи-макрос (Emoji)
    // Проверяем: работу со слипшимся синтаксисом (унарный минус прямо перед X, 
    // деление прямо перед числом) на кастомном словаре!
    let emoji_macro_program = "✍️ X = 10\n✍️ Y = -X + 5!\n🖨 X\n🖨 Y\n✍️ X = Y / 2\n🖨 X";
    
    println!("\n--- ЗАПУСК ТЕСТА 2 (Emoji) ---");
    run(emoji_macro_program, Dictionaries::Emoji);
}

