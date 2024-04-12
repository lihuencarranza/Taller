use regular_expressions::create_regular_expressions;
use tp1::{matching::{self}, regular_expressions};
use std::env;
use matching::compare_regexes_with_expression;

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
    
    let mut result = Vec::new();
    for s in list.iter() {
        let word = compare_regexes_with_expression(&regexes, s.to_string());
        if !word.is_err() {
            result.push(word);
        }
    }

    for r in result.iter() {
        if let Ok(s) = r {
            println!("{}", s.trim());
        }
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

