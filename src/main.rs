use basic_lexer::settings::{
    run
};

use basic_lexer::main_logic::syntaxd::Dictionaries;

fn main() {
    let program = "
        RANDOM SECRET 1 100
        LET TRIES = 0

        PRINT \"--- GUESS THE NUMBER GAME ---\"

        :game_loop
        PRINT \"Enter your guess:\"
        INPUT GUESS

        LET TRIES = TRIES + 1

        IF GUESS == SECRET THEN GOTO win
        IF GUESS < SECRET THEN GOTO too_low
        IF GUESS > SECRET THEN GOTO too_high

        :too_low
        PRINT \"Too low! Try again.\"
        GOTO game_loop

        :too_high
        PRINT \"Too high! Try again.\"
        GOTO game_loop

        :win
        PRINT \"YOU WIN!!!\"
        PRINT \"Total tries:\"
        PRINT TRIES
    ";

    run(program, Dictionaries::English);
}
