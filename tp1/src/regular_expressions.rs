use crate::regex::Regex;

/// Function to create regular expressions
/// It receives a string and returns a vector of regular expressions
/// # Example
/// receives "a|b" and returns Ok(vec![Regex { regex: "a" }, Regex { regex: "b" }])
pub fn create_regular_expressions(expression: &str) -> Result<Vec<Regex>, &'static str> {
    if expression.is_empty() {
        return Err("Empty expression");
    }
    let parts: Vec<&str> = expression.split('|').collect();
    let mut regexes: Vec<Regex> = Vec::new();
    for part in parts {
        let regex = Regex::new(part);
        match regex {
            Ok(r) => regexes.push(r),
            Err(_) => break,
        }
    }
    Ok(regexes)
}
