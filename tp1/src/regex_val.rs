use crate::metachars::RegexClass;

#[derive(Debug, PartialEq)]
pub enum RegexValue {
    Literal(char),
    Wildcard,
    Class(RegexClass),
    OneOf(Vec<char>),
}