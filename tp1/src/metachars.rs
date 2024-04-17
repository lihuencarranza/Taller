/// Enum to represent the different classes of characters
#[derive(Debug, PartialEq)]
pub enum RegexClass {
    /// Represents the class of alphabetic characters
    Alpha,
    /// Represents the class of alphanumeric characters
    Alnum,
    /// Represents the class of digit characters
    Digit,
    /// Represents the class of lowercase characters
    Lower,
    /// Represents the class of uppercase characters
    Upper,
    /// Represents the class of punctuation characters
    Space,
    /// Represents the class of space characters
    Punct,
}

/// Function to handle metacharacters
/// - It receives a string and returns a Result with the RegexClass or an error
/// # Arguments
/// * `n` - A string that represents a metacharacter
/// # Returns
/// * A Result with the RegexClass or an error
/// # Example
/// let result = handle_metachar(":alpha:".to_string());
/// assert_eq!(result, Ok(RegexClass::Alpha));
pub fn handle_metachar(n: String) -> Result<RegexClass, &'static str> {
    Ok(match n.as_str() {
        ":alpha:" => RegexClass::Alpha,
        ":alnum:" => RegexClass::Alnum,
        ":digit:" => RegexClass::Digit,
        ":lower:" => RegexClass::Lower,
        ":upper:" => RegexClass::Upper,
        ":punct:" => RegexClass::Punct,
        ":space:" => RegexClass::Space,
        _ => return Err("Invalid metacharacter"),
    })
}
