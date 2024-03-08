use crate::utils::*;
use std::fmt::Write;
use std::str::SplitAsciiWhitespace;

pub fn solve(data: String) -> String {
    let mut input = String::new();
    let mut output = String::new();
    let mut tokens: SplitAsciiWhitespace;
    if data == STDIN {
        tokens = load_tokens(&mut input);
    } else {
        tokens = data.SplitAsciiWhitespace();
    }
    let n: u64 = get_token(&mut tokens);
    let mut s: u64 = (n * (n + 1)) / 2;
    let v: Vec<u64> = (0..n - 1).map(|_| get_token(&mut tokens)).collect();
    for e in v {
        s -= e;
    }
    writeln!(output, "{s}").unwrap();
    output
}


#[cfg(test)]
mod tests {
    use crate::missing_number::solve;
    use crate::utils::test_setup;

    #[test]
    pub fn run() {
        use std::fs;
        let (questions, answers) = test_setup(String::from("test_data/missing_number"));
        for (question, answer) in questions.iter().zip(answers) {
            let mut input = fs::read_to_string(question)
                .expect(question);
            let answer = answer.as_str();
            let result = fs::read_to_string(answer)
                .expect(answer);
            let output = solve(input);
            assert_eq!(output.trim(), result.trim());
        }
    }
}