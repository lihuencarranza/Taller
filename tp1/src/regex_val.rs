use crate::metachars::RegexClass;

/// Enum to represent a regex value
#[derive(Debug, PartialEq)]
pub enum RegexValue {
    Literal(char),
    Wildcard,
    Class(RegexClass),
    OneOf(Vec<char>),
}