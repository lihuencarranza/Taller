use matching::compare_regexes_with_expression;
use regular_expressions::create_regular_expressions;
use std::env;
use tp1::{
    matching::{self},
    regular_expressions,
};

/// Read a file and return its content as a string
fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

/// Create a list of strings from a file
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

// Define a new type for the complex type
type ResultVec =
    Result<Vec<Result<String, Box<dyn std::error::Error>>>, Box<dyn std::error::Error>>;

/// Process the expressions and paths
fn process_expressions_and_paths(expression: &str, path: &str) -> ResultVec {
    let regexes = create_regular_expressions(expression)?;
    let list = create_list_from_file(path);

    let mut result = Vec::new();
    for s in list.iter() {
        let word = compare_regexes_with_expression(&regexes, s.to_string()).map_err(|e| e.into());
        // Use word.is_ok() instead of !word.is_err()
        if word.is_ok() {
            result.push(word);
        }
    }
    Ok(result)
}

/// Print the results
fn print_results(results: &[Result<String, Box<dyn std::error::Error>>]) {
    for s in results.iter().flatten() {
        println!("{}", s.trim());
    }
}

/// Parse the arguments
fn parse_args() -> Result<(String, String), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("Expected at least two arguments".into());
    }

    let expression = &args[1];
    if expression.is_empty() {
        return Err("Expression is empty".into());
    }

    let path = &args[2];
    if path.is_empty() {
        return Err("Path is empty".into());
    }

    Ok((expression.to_string(), path.to_string()))
}

///  This program implements the egrep command
///
/// # How does it work?
/// The program receives an expression and a path to a file. It reads the file and compares each line with the expression.
/// It creates a list of regular expressions from the expression and then compares each line with the regular expressions.
/// If the line matches the regular expressions, it prints the line.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (expression, path) = parse_args()?;
    if expression.is_empty() || path.is_empty() {
        return Err("Empty expression or path".into());
    }

    let results = process_expressions_and_paths(&expression, &path)?;
    print_results(&results);

    Ok(())
}
