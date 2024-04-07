use crate::regex::RegexStep;

pub fn handle_start_of_line(backtracking: &mut bool) -> Result<Option<RegexStep>, &'static str> {
    *backtracking = false;
    Ok(None)
}