use crate::regex_rep::RegexRep;
use crate::regex_val::RegexValue;

/// Struct to represent a regex step
/// - It has a RegexValue and a RegexRep
#[derive(Debug, PartialEq)]
pub struct RegexStep {
    /// The value of the regex step {literal, wildcard, class, oneof}
    pub val: RegexValue,
    /// The repetition of the regex step {exact, range, none}
    pub rep: RegexRep,
}
