use crate::regex::Regex;

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
