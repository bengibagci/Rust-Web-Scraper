use std::fs::OpenOptions;
use std::io::Write;

pub fn save_to_file(file_path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .expect("Cannot open file");

    file.write_all(content.as_bytes()).expect("Cannot write to file");
}