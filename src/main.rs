use basic_lexer::settings::{
    run
};

use basic_lexer::main_logic::syntaxd::Dictionaries;

fn main() {
    let program = "
        LET result = (5 + 4 + 4!)^2 * 89
        PRINT result
    "; // 417210394672

    run(program, Dictionaries::English);
}