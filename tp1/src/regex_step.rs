use crate::regex_rep::RegexRep;
use crate::regex_val::RegexValue;


#[derive(Debug, PartialEq)]
pub struct RegexStep {
    pub val: RegexValue,
    pub rep: RegexRep,
}