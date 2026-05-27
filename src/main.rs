use basic_lexer::settings::{
    run,
    scan_code,
    load_code,
};

use basic_lexer::main_logic::syntaxd::{Dictionaries, SyntaxDict};


fn main() {
    // Find files in dir FILES
    let content_to_load = match scan_code("src/FILES") {
        Ok(files) => files,
        Err(e) => {
            eprintln!("[Scanning error]: {}", e);
            return;
        }
    };

    // 2. Get file.bsa
    let path = match content_to_load.first() {
        Some(p) => p,
        None => { 
            eprintln!("[Error]: No files with extension found in folder 'FILES' .bsa"); 
            return; 
        }
    };

    // 3. Loading the code from the file
    let mut code = match load_code(path) {
        Ok(text) => text,
        Err(e) => { 
            eprintln!("[Error reading file {:?}]: {}", path, e); 
            return; 
        }
    };
    // Clean code from Windows /r and emoji unicodes
    code = code.replace("\r", "").replace("\u{fe0f}", "");
    let mut config = Dictionaries::English;

    // Check the kind of Dict we need to use
    // Внутри main.rs, где работает препроцессор:
    if let Some(first_line) = code.lines().next() {
        let trimmed = first_line.trim();
        
        if trimmed.starts_with("#mode") {
            if let Some(start_quote) = trimmed.find('"') {
                if let Some(end_quote) = trimmed.rfind('"') {
                    if start_quote != end_quote {
                        let dict_name = &trimmed[start_quote + 1..end_quote];
                        config = SyntaxDict::get_dict(dict_name); 
                        println!("[Preprocessor]: Dictionary for language successfully connected: {}", dict_name);
                    }
                }
            }
            // Отрезаем директиву подключения из кода
            if let Some(pos) = code.find('\n') {
                code = code[pos + 1..].to_string();
            }
        }
    }
    println!("Launching file: {:?}", path);
    println!("-----------------------------------------");

    // 7. Запускаем твой интерпретатор с динамически выбранным синтаксисом!
    if !code.is_empty() {
        run(&code, config);
    }
}
