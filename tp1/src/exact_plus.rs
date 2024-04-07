use crate::regex::RegexRep;
use crate::regex::RegexStep;

pub fn handle_exact_plus(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range { min: Some(1), max: None };
    } else {
        return Err("Unexpected '+' character");
    }
    Ok(None)
}
