use chrono::{DateTime, Utc};
use md5;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

const BASE_LINK: &str = "http://api.smitegame.com/smiteapi.svc";

fn read_file_to_string(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

fn read_secret(secret_key: &str) -> String {
    let token_file = read_file_to_string("resources/token.json").unwrap();
    let json: Value = serde_json::from_str(&token_file).unwrap();

    json[secret_key].as_str().unwrap().to_string()
}

fn get_formatted_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y%m%d%H%M%S").to_string()
}

fn make_signature(devid: &str, methodname: &str, token: &str, time: &str) -> String {
    let unhashed_signature = format!("{}{}{}{}", devid, methodname, token, time);
    let bytes = unhashed_signature.as_bytes();
    format!("{:x}", md5::compute(bytes))
}

pub fn create_session() {
    let devid = read_secret("devid");
    let method = "createsession";
    let token = read_secret("token");
    let time = get_formatted_time();
    let signature = make_signature(&devid, method, &token, &time);

    let link = format!(
        "{}/{}json/{}/{}/{}",
        BASE_LINK, method, devid, signature, time
    );
    println!("{}", link);
}
