use crate::regex::RegexRep;
use crate::regex::RegexStep;
use crate::regex::RegexValue;

pub fn handle_wildcard() -> Option<RegexStep> {
    Some(RegexStep{ rep: RegexRep::Exact(1), val: RegexValue::Wildcard })
}