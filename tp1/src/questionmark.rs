use crate::regex::RegexRep;
use crate::regex::RegexStep;

pub fn handle_zero_or_one(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range { min: None, max: Some(1) };
    } else {
        return Err("Unexpected '?' character");
    }
    Ok(None)
}
