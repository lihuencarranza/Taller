use crate::regex::RegexRep;
use crate::regex::RegexStep;

pub fn handle_any(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Any;
    } else {
        return Err("Unexpected '*' character");
    }
    Ok(None)
}
