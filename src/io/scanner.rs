use std::fs;
use std::path::PathBuf;

pub fn scan(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    // read_dir(dir)? — знак вопроса сразу вернет ошибку наружу, если папки нет
    let entries = fs::read_dir(dir)?;

    let results = entries
        .filter_map(|entry| entry.ok()) 
        .map(|e| e.path())              
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("bsa")
        })
        .collect(); 

    Ok(results)
}