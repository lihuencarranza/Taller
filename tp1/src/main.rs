use regular_expressions::create_regular_expressions;
use std::env;
use tp1::{match_regex::{self, compare_regex_with_expression}, regex, regular_expressions};
//use match_regex::compare_regex_with_expression;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (expression, path) = parse_args()?;
    if expression.is_empty() || path.is_empty() {
        return Err("Empty expression or path".into());
    }

    let regexes = create_regular_expressions(&expression);
    let regexes = match regexes {
        Ok(r) => r,
        Err(e) => return Err(e.into()),
    };

    let list = create_list_from_file(&path);
    println!("{:?}\n", list);

    for r in regexes.iter() {
        println!("{:?}", r);
    }

    let mut result = Vec::new();
    for s in list.iter() {
        let new = compare_regex_with_expression(&regexes, s.to_string());
        result.extend(new);
    }

    for r in result.iter() {
        println!("{:?} in {:?}", r.matched, r.expression);
    }
    
    Ok(())
}

fn parse_args() -> Result<(String, String), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("Expected at least two arguments".into());
    }

    let expression = args[1].clone();
    if expression.is_empty() {
        return Err("Expression is empty".into());
    }

    let path = args[2].clone();
    if path.is_empty() {
        return Err("Path is empty".into());
    }

    Ok((expression, path))
}

fn read_file(path: &str) -> String {
    let file = std::fs::read_to_string(path);
    let file = match file {
        Ok(f) => f,
        Err(e) => panic!("Error reading file: {}", e),
    };
    file
}

fn create_list_from_file(path: &str) -> Vec<String> {
    let file = read_file(path);
    let mut list: Vec<String> = Vec::new();
    let mut word = String::new();
    for c in file.chars() {
        if c == '\n' {
            list.push(word);
            word = String::new();
        } else {
            word.push(c);
        }
    }
    list
}

/*#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_create_list_from_file() {
                let path = "texto.txt";
                let expected = vec!["abbcd", "ab", "abcd ", "ac", "bc", "dd", "cdab", "abgcd", "abggcd"];
                let result = create_list_from_file(path);
                assert_eq!(result, expected);
        }
}*/
