use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use crate::regex_val::RegexValue;
use std::str::Chars;

pub fn handle_escape_sequence(chars_iter: &mut Chars) -> Result<Option<RegexStep>, &'static str> {
    let c = chars_iter
        .next()
        .ok_or("Se esperaba un caracter después de \\")?;
    Ok(Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexValue::Literal(c),
    }))
}

pub fn handle_wildcard() -> Option<RegexStep> {
    Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexValue::Wildcard,
    })
}
