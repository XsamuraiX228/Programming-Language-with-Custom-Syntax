use basic_lexer::{io::scanner::{
    load_code,
    scan_code,}, 
    run_pipeline
};

fn main() -> Result<(), String> {
    // Find files in dir FILES
    let content_to_load = match scan_code("examples") {
        Ok(files) => files,
        Err(e) => {
            return Err(format!("[Scanning error]: {}", e));
        }
    };

    // 2. Get file.bsa
    let path = match content_to_load.first() {
        Some(p) => p,
        None => { 
            return Err(format!("[Error]: No files with extension found in folder 'FILES' .bsa")); 
        }
    };

    // 3. Loading the code from the file
    let code = match load_code(path) {
        Ok(text) => text,
        Err(e) => { 
            return Err(format!("[Error reading file {:?}]: {}", path, e)); 
        }
    };
    
    run_pipeline(&code)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_program {
        ($name:ident, $program:expr) => {
            #[test]
            fn $name() {
                let result = run_pipeline($program);
                assert!(result.is_ok(), "Program failed: {:?}", result);
            }
        };
    }

    macro_rules! test_error {
        ($name:ident, $program:expr) => {
            #[test]
            fn $name() {
                let result = run_pipeline($program);
                assert!(result.is_err(), "Program should have failed");
            }
        };
    }

    // ==================== ARYTHMETIC ====================
    
    test_program!(test_addition, "
        LET X = 5 + 3
    ");
    
    test_program!(test_subtraction, "
        LET X = 10 - 4
    ");
    
    test_program!(test_multiplication, "
        LET X = 7 * 6
    ");
    
    test_program!(test_division, "
        LET X = 15 / 3
    ");
    
    test_program!(test_power, "
        LET X = 2 ^ 4
    ");
    
    test_program!(test_factorial, "
        LET X = 5!
    ");
    
    test_program!(test_precedence, "
        LET X = 5 + 3 * 2
    ");
    
    test_program!(test_parentheses, "
        LET X = (5 + 3) * 2
    ");
    
    // ==================== VARIABLES ====================
    
    test_program!(test_variable, "
        LET A = 10
        LET B = A + 5
    ");
    
    test_program!(test_reassignment, "
        LET X = 10
        LET X = X + 5
    ");
    
    // ==================== IF-ELSE ====================
    
    test_program!(test_if_true, "
        LET X = 10
        IF X == 10 THEN
            LET Y = 1
        END
    ");
    
    test_program!(test_if_false, "
        LET X = 5
        IF X == 10 THEN
            LET Y = 1
        END
    ");
    
    test_program!(test_if_else, "
        LET X = 5
        IF X == 10 THEN
            LET Y = 1
        ELSE
            LET Y = 2
        END
    ");
    
    test_program!(test_if_less, "
        LET X = 5
        IF X < 10 THEN
            LET Y = 1
        END
    ");
    
    test_program!(test_if_greater, "
        LET X = 15
        IF X > 10 THEN
            LET Y = 1
        END
    ");
    
    test_program!(test_if_less_equal, "
        LET X = 10
        IF X <= 10 THEN
            LET Y = 1
        END
    ");
    
    test_program!(test_if_greater_equal, "
        LET X = 10
        IF X >= 10 THEN
            LET Y = 1
        END
    ");
    
    test_program!(test_nested_if, "
        LET X = 10
        LET Y = 20
        IF X == 10 THEN
            IF Y == 20 THEN
                LET Z = 1
            END
        END
    ");
    
    test_program!(test_nested_if_else, "
        LET X = 10
        LET Y = 5
        IF X == 10 THEN
            IF Y == 20 THEN
                LET Z = 1
            ELSE
                LET Z = 2
            END
        END
    ");
    
    test_program!(test_if_chain, "
        LET SCORE = 75
        IF SCORE >= 90 THEN
            LET GRADE = 5
        ELSE
            IF SCORE >= 80 THEN
                LET GRADE = 4
            ELSE
                IF SCORE >= 70 THEN
                    LET GRADE = 3
                ELSE
                    LET GRADE = 2
                END
            END
        END
    ");
    
    // ==================== WHILE LOOPS ====================
    
    test_program!(test_while_basic, "
        LET I = 1
        WHILE I <= 5 THEN
            LET I = I + 1
        WEND
    ");
    
    test_program!(test_while_with_if, "
        LET I = 1
        LET SUM = 0
        WHILE I <= 10 THEN
            IF I % 2 == 0 THEN
                LET SUM = SUM + I
            END
            LET I = I + 1
        WEND
    ");
    
    test_program!(test_nested_while, "
        LET I = 1
        WHILE I <= 3 THEN
            LET J = 1
            WHILE J <= 3 THEN
                LET K = I * J
                LET J = J + 1
            WEND
            LET I = I + 1
        WEND
    ");
    
    test_program!(test_while_never_enters, "
        LET I = 10
        WHILE I < 5 THEN
            LET X = 1
        WEND
    ");
    
    // ==================== GOTO AND MARKS ====================
    
    test_program!(test_goto, "
        GOTO skip
        LET X = 1
        :skip
        LET Y = 2
    ");
    
    test_program!(test_goto_loop, "
        LET X = 1
        :start
        LET X = X + 1
        IF X <= 3 THEN GOTO start
        END
    ");
    
    // ==================== RANDOM ====================
    
    test_program!(test_random, "
        RANDOM X 1 100
    ");
    
    // ==================== TESTS FOR ERROS ====================
    
    test_error!(test_division_by_zero, "
        LET X = 10 / 0
    ");
    
    test_error!(test_undefined_variable, "
        PRINT UNDEFINED
    ");
    
    test_error!(test_goto_missing_label, "
        GOTO missing_label
    ");
    
    test_error!(test_missing_wend, "
        WHILE X < 10 THEN
            LET X = X + 1
    ");
    
    test_error!(test_missing_end, "
        IF X == 10 THEN
            LET X = X + 1
    ");
    
    // ==================== COMPLEX TESTS ====================
    
    test_program!(test_fibonacci, "
        LET A = 0
        LET B = 1
        LET N = 10
        LET I = 0
        
        WHILE I < N THEN
            LET C = A + B
            LET A = B
            LET B = C
            LET I = I + 1
        WEND
    ");
    
    test_program!(test_factorial_loop, "
        LET N = 5
        LET FACT = 1
        WHILE N > 0 THEN
            LET FACT = FACT * N
            LET N = N - 1
        WEND
    ");
    
    test_program!(test_prime_check, "
        LET NUM = 17
        LET IS_PRIME = 1
        LET I = 2
        
        WHILE I * I <= NUM THEN
            IF NUM % I == 0 THEN
                LET IS_PRIME = 0
            END
            LET I = I + 1
        WEND
    ");
}