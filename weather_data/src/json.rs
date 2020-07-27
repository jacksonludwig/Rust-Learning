use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

fn read_file_to_string(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

pub fn read_token() -> String {
    let token_file = read_file_to_string("resources/token.json").unwrap();
    let json: Value = serde_json::from_str(&token_file).unwrap();

    json["token"].as_str().unwrap().to_string()
}
