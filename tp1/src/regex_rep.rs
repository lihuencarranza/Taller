
/// Represents the repetition of a regex.
#[derive(Debug, PartialEq)]
pub enum RegexRep {
    Exact(usize),
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
    None,
}