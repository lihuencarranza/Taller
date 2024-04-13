use crate::regex::{RegexRestriction, RegexStep};

pub fn handle_start_of_line(backtracking: &mut Option<Vec<RegexRestriction>>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(backtracking_vec) = backtracking {
        backtracking_vec.push(RegexRestriction::StartOfLine);
    }
    Ok(None)
}