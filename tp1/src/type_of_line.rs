use crate::regex_step::RegexStep;

#[derive(Debug, PartialEq)]
pub enum RegexRestriction {
    StartOfLine,
    EndOfLine,
    None,
}

pub fn handle_start_of_line(backtracking: &mut Option<Vec<RegexRestriction>>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(backtracking_vec) = backtracking {
        backtracking_vec.push(RegexRestriction::StartOfLine);
    }
    Ok(None)
}

pub fn handle_end_of_line(backtracking: &mut Option<Vec<RegexRestriction>>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(backtracking_vec) = backtracking {
        backtracking_vec.push(RegexRestriction::EndOfLine);
    }
    Ok(None)
}