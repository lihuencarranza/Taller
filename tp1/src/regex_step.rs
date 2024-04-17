use crate::regex_rep::RegexRep;
use crate::regex_val::RegexValue;

/// Struct to represent a regex step
/// It has a RegexValue and a RegexRep
#[derive(Debug, PartialEq)]
pub struct RegexStep {
    pub val: RegexValue,
    pub rep: RegexRep,
}
