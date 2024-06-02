- This is the challenge [here](https://play.picoctf.org/practice/challenge/104?category=3&page=1)
- Practice with this [walkthrough](https://medium.com/@andrewss112/picoctf-write-up-transformation-7d458148e802)
---

# Test encoding function
![alt text](./first.png)
```rs
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("File location:\n{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");

    println!("With text:\n{contents}");

    let encoded_txt = encode_ascii_text(&contents);
    println!("Encoded text:\n{encoded_txt}");
}

```