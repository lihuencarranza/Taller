use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use crate::regex_val::RegexValue;
use std::str::Chars;

/// Function to handle the escape sequence
/// - It receives a mutable reference to Chars and returns a Result with a RegexStep or an error
/// # Arguments
/// * `chars_iter` - A mutable reference to Chars
/// # Returns
/// * A Result with a RegexStep or an error
/// # Example
/// let mut chars = "\\a".chars();
/// let result = handle_escape_sequence(&mut chars);
/// assert_eq!(result, Ok(Some(RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') })));
pub fn handle_escape_sequence(chars_iter: &mut Chars) -> Result<Option<RegexStep>, &'static str> {
    let c = chars_iter
        .next()
        .ok_or("Se esperaba un caracter después de \\")?;
    Ok(Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexValue::Literal(c),
    }))
}

/// Function to handle the wildcard
/// It returns a RegexStep
/// # Arguments
/// * `None`
/// # Returns
/// * A RegexStep
/// # Example
/// let result = handle_wildcard();
/// assert_eq!(result, Some(RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Wildcard }));
pub fn handle_wildcard() -> Option<RegexStep> {
    Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexValue::Wildcard,
    })
}
