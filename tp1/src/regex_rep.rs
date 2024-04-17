/// Represents the repetition of a regex.
/// - It can be an exact number of times, a range of times or none.
#[derive(Debug, PartialEq)]
pub enum RegexRep {
    /// Represents the repetition of a regex's step exactly n times
    Exact(usize),
    /// Represents the repetition of a regex's step at least n times to m times
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
    /// Represents the repetition of a regex's step that is value is not in the word
    None,
}
