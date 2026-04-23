use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Input {
    pattern: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct Output {
    ok: bool,
    matched: bool,
}

fn main() {
    let mut stdin = String::new();
    std::io::stdin()
        .read_to_string(&mut stdin)
        .expect("failed to read stdin");

    let input: Input = serde_json::from_str(&stdin).expect("failed to parse stdin JSON");
    let regex = Regex::new(&input.pattern).expect("invalid regex pattern");
    let output = Output {
        ok: true,
        matched: regex.is_match(&input.text),
    };

    println!(
        "{}",
        serde_json::to_string(&output).expect("failed to serialize output")
    );
}
