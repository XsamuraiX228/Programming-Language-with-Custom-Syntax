pub fn load(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
