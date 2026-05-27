use std::path::PathBuf;

pub fn scan(dir: &str) -> Vec<PathBuf>{
    let mut results = Vec::new();

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return results,
    };
    

    for entry in entries {
        let e = match entry { Ok(e) => e, Err(_) => continue };
        let path = e.path();

        let matches = path 
            .extension()
            .map_or(false, |ext| ext == "bsa");

        if !matches { println!("NOT MATCHING"); }

        if path.is_file() && matches {
                results.push(path);
                println!("NEW FILE LOADED")
        }
    }

    return results;
}
