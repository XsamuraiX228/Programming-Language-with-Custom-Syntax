use basic_lexer::settings::{
    run
};

use basic_lexer::main_logic::syntaxd::Dictionaries;

fn main() {
    let program = "LET x = 5! * 6 + (6+9)\nPRINT x";
    run(program, Dictionaries::English);
    let smile_program = "✍️ X = -(-2)^3! + (4 * (1 + 2)) / -2\n🖨 X";
    run(smile_program, Dictionaries::Emoji);
}

