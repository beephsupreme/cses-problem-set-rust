use std::{io::{BufReader, Read}, str::{self, SplitAsciiWhitespace}};

pub const STDIN: &str = "stdin";
pub fn get_token<T: str::FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    if let Some(token) = tokens.next() {
        token.parse::<T>().ok().unwrap_or_else(|| {
            panic!(
                "parse fail: \"{}\" -> {}",
                token,
                std::any::type_name::<T>()
            )
        })
    } else {
        panic!("parse fail: None -> {}", std::any::type_name::<T>())
    }
}

pub fn load_tokens(buffer: &mut String) -> SplitAsciiWhitespace {
    let mut reader = BufReader::new(std::io::stdin());
    reader.read_to_string(buffer).expect("could not read from stdin");
    buffer.split_ascii_whitespace()
}

pub fn test_setup(test_data: String) -> (Vec<String>, Vec<String>) {
    use std::fs;
    let paths = fs::read_dir(test_data).unwrap();
    let mut input: Vec<String> = Vec::new();
    let mut output: Vec<String> = Vec::new();

    for path in paths {
        let file = path.unwrap().path().display().to_string();
        if file.contains("input") {
            input.push(file);
        } else {
            output.push(file);
        }
    }
    input.sort();
    output.sort();
    (input, output)
}

