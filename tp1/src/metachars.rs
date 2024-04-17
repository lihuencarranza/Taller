/// Enum to represent the different classes of characters
#[derive(Debug, PartialEq)]
pub enum RegexClass {
    Alpha,
    Alnum,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
}

/// Function to handle metacharacters
/// It receives a string and returns a Result with the RegexClass or an error
/// # Example
/// receives ":alpha:" and returns Ok(RegexClass::Alpha)
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
