use crate::regex::Regex;

/// Function to create regular expressions
/// - It receives a string and returns a vector of regular expressions
/// # Arguments
/// * `expression` - A string that represents a regular expression
/// # Returns
/// * A vector of regular expressions or an error
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

#[cfg(test)]
mod regexes_creation_tests {
    use crate::{regex_rep::RegexRep, regex_step::RegexStep, regex_val::RegexValue};

    use super::*;

    #[test]
    fn test_1() {
        let expression = "a|b|c";
        let result = create_regular_expressions(expression);
        assert_eq!(
            result,
            Ok(vec![
                Regex {
                    steps: vec![RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('a')
                    }],
                    backtracking: None
                },
                Regex {
                    steps: vec![RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('b')
                    }],
                    backtracking: None
                },
                Regex {
                    steps: vec![RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('c')
                    }],
                    backtracking: None
                }
            ])
        );
    }

    #[test]
    fn test_2() {
        let expression = "";
        let result = create_regular_expressions(expression);
        assert_eq!(result, Err("Empty expression"));
    }
}
