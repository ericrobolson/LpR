use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::io::BufReader;
use std::io::Read;

fn to_markdown() {
    unimplemented!();
}

fn open(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;

    return Ok(io::BufReader::new(file).lines());
}

fn to_rust(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> String {
    //let lines = open(Path::new("lib.rs")).unwrap();

    let mut output = String::new();

    for line in lines {
        let line = line.unwrap();
        output.push_str(&line.to_string());
    }

    return output;

    // for line in lines {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_rust_() {
        let data = BufReader::new(
            "
        #This is a test.

        ```
        mod parser;

        pub fn lpr() {
            let t = true;
        }
        ```
        "
            .as_bytes(),
        );

        let expected = "
        // This is a test.
        
        mod parser;

        pub fn lpr() {
            let t = true;
        }        
        ";

        let actual = to_rust(data.lines());

        assert_eq!(expected, actual);
    }
}
