use crate::regex_step::RegexStep;

/// Enum to represent a regex restriction
#[derive(Debug, PartialEq)]
pub enum RegexRestriction {
    StartOfLine,
    EndOfLine,
    None,
}

/// Function to handle the start of line metacharacter
/// It receives a mutable reference to an Option of a vector of RegexRestriction and returns a Result with an Option of a RegexStep or an error
/// # Example
/// receives a mutable reference to an Option of a vector of RegexRestriction and returns Ok(None)
pub fn handle_start_of_line(
    backtracking: &mut Option<Vec<RegexRestriction>>,
) -> Result<Option<RegexStep>, &'static str> {
    if let Some(backtracking_vec) = backtracking {
        backtracking_vec.push(RegexRestriction::StartOfLine);
    }
    Ok(None)
}

/// Function to handle the end of line metacharacter
/// It receives a mutable reference to an Option of a vector of RegexRestriction and returns a Result with an Option of a RegexStep or an error
/// # Example
/// receives a mutable reference to an Option of a vector of RegexRestriction and returns Ok(None)
pub fn handle_end_of_line(
    backtracking: &mut Option<Vec<RegexRestriction>>,
) -> Result<Option<RegexStep>, &'static str> {
    if let Some(backtracking_vec) = backtracking {
        backtracking_vec.push(RegexRestriction::EndOfLine);
    }
    Ok(None)
}
