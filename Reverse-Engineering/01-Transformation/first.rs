use std::env;
use std::fs;

fn encode_ascii_text(text: &str) -> String {
    let mut encoded_text = String::new();

    for c in text.chars() {
        let ascii_code = c as u32;
        let hex_code = format!("{:X}", ascii_code);
        encoded_text.push_str(&hex_code);
    }
    encoded_text
}

fn decode_ascii_text(encoded_text: &str) -> String {
    let bytes = (0..encoded_text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&encoded_text[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();

    String::from_utf8_lossy(&bytes).into_owned()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("File location:\n{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");

    println!("With text:\n{contents}");

    let encoded_txt = encode_ascii_text(&contents);
    println!("Encoded text:\n{encoded_txt}");

    let decoded_txt = decode_ascii_text(&encoded_txt);
    println!("Decoded text:\n{:?}", decoded_txt);
}
