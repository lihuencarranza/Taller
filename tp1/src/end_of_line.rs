use crate::regex::{RegexRestriccion, RegexStep};

pub fn handle_end_of_line(backtracking: &mut Option<Vec<RegexRestriccion>>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(backtracking_vec) = backtracking {
        backtracking_vec.push(RegexRestriccion::EndOfLine);
    }
    Ok(None)
}