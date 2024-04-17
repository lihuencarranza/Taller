use crate::metachars::RegexClass;

/// Enum to represent a regex value
#[derive(Debug, PartialEq)]
pub enum RegexValue {
    /// Represents a literal character
    Literal(char),
    /// Represents a wildcard character
    Wildcard,
    /// Represents a class of characters
    Class(RegexClass),
    /// Represents one of the characters in the vector
    OneOf(Vec<char>),
}
