use crate::regex::RegexRep;
use crate::regex::RegexStep;
use crate::regex::RegexValue;
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