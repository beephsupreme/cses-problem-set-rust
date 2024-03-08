use std::fmt::Write;

pub fn solve() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut n: u64 = input.trim().parse().unwrap();
    let mut buffer = String::new();
    loop {
        write!(buffer, "{n} ").unwrap();
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    println!("{}", buffer);
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn solve() {
        use std::fs;
        use std::fmt::Write;
        let test_data = String::from("test_data/weird_algorithm");
        let paths = fs::read_dir(test_data).unwrap();
        let mut in_files: Vec<String> = Vec::new();
        let mut out_files: Vec<String> = Vec::new();
        
        for path in paths {
            let file = path.unwrap().path().display().to_string();
            if file.contains("input") {
                in_files.push(file);
            } else {
                out_files.push(file);
            }
        }
        
        in_files.sort();
        out_files.sort();

        for (infile, resfile) in in_files.iter().zip(out_files) {
            let input = fs::read_to_string(infile)
                .expect(infile);
            let result = fs::read_to_string(resfile.as_str())
                .expect(resfile.as_str());
            let mut n: u64 = input.trim().parse().unwrap();
            let mut output = String::new();
            loop {
                write!(output, "{n} ").unwrap();
                if n == 1 {
                    break;
                }
                if n % 2 == 0 {
                    n /= 2;
                } else {
                    n = 3 * n + 1;
                }
            }
            assert_eq!(output.trim(), result.trim());
        }
    }
}