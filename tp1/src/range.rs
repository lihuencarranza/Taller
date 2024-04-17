use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use std::str::Chars;

/// Function to handle the any metacharacter
/// - It receives a mutable reference to a vector of RegexStep and returns a Result with a RegexStep or an error
/// # Arguments
/// * `steps` - A mutable reference to a vector of RegexStep
/// # Returns
/// * A Result with a RegexStep or an error
/// # Example
/// let mut steps = vec![RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') }];
/// let result = handle_any(&mut steps);
/// assert_eq!(result, Ok(None));
/// assert_eq!(steps, vec![RegexStep { rep: RegexRep::Range { min: None, max: None }, val: RegexValue::Literal('a') }]);
pub fn handle_any(steps: &mut [RegexStep]) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range {
            min: None,
            max: None,
        };
    } else {
        return Err("Unexpected '*' character");
    }
    Ok(None)
}

/// Function to handle the zero or one metacharacter
/// - It receives a mutable reference to a vector of RegexStep and returns a Result with a RegexStep or an error
/// # Arguments
/// * `steps` - A mutable reference to a vector of RegexStep
/// # Returns
/// * A Result with a RegexStep or an error
/// # Example
/// let mut steps = vec![RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') }];
/// let result = handle_zero_or_one(&mut steps);
/// assert_eq!(result, Ok(None));
/// assert_eq!(steps, vec![RegexStep { rep: RegexRep::Range { min: None, max: Some(1) }, val: RegexValue::Literal('a') }]);
pub fn handle_zero_or_one(steps: &mut [RegexStep]) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range {
            min: None,
            max: Some(1),
        };
    } else {
        return Err("Unexpected '?' character");
    }
    Ok(None)
}

/// Function to handle the one or more metacharacter
/// - It receives a mutable reference to a vector of RegexStep and returns a Result with a RegexStep or an error
/// # Arguments
/// * `steps` - A mutable reference to a vector of RegexStep
/// # Returns
/// * A Result with a RegexStep or an error
/// # Example
/// let mut steps = vec![RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') }];
/// let result = handle_exact_plus(&mut steps);
/// assert_eq!(result, Ok(None));
/// assert_eq!(steps, vec![RegexStep { rep: RegexRep::Range { min: Some(1), max: None }, val: RegexValue::Literal('a') }]);
pub fn handle_exact_plus(steps: &mut [RegexStep]) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range {
            min: Some(1),
            max: None,
        };
    } else {
        return Err("Unexpected '+' character");
    }
    Ok(None)
}

/// Function to handle the range metacharacter
/// - It receives a mutable reference to Chars and a mutable reference to a vector of RegexStep and returns a Result with a RegexStep or an error
/// # Arguments
/// * `chars_iter` - A mutable reference to Chars
/// * `steps` - A mutable reference to a vector of RegexStep
/// # Returns
/// * A Result with a RegexStep or an error
/// # Example
/// let mut chars = "1,2}".chars();
/// let mut steps = vec![RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') }];
/// let result = handle_range(&mut chars, &mut steps);
/// assert_eq!(result, Ok(None));
/// assert_eq!(steps, vec![RegexStep { rep: RegexRep::Range { min: Some(1), max: Some(2) }, val: RegexValue::Literal('a') }]);
pub fn handle_range(
    chars_iter: &mut Chars,
    steps: &mut [RegexStep],
) -> Result<Option<RegexStep>, &'static str> {
    let mut n = String::new();
    for c in chars_iter.by_ref() {
        if c == '}' {
            break;
        }
        n.push(c);
    }
    let parts: Vec<&str> = n.split(',').collect();
    if let Some(last) = steps.last_mut() {
        match parts.len() {
            1 => {
                let exact = parts[0]
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse exact repetition")?;
                last.rep = RegexRep::Exact(exact);
            }
            2 => {
                let min = if parts[0].is_empty() {
                    None
                } else {
                    Some(
                        parts[0]
                            .parse::<usize>()
                            .map_err(|_| "Failed to parse min repetition")?,
                    )
                };
                let max = if parts[1].is_empty() {
                    None
                } else {
                    Some(
                        parts[1]
                            .parse::<usize>()
                            .map_err(|_| "Failed to parse max repetition")?,
                    )
                };
                last.rep = RegexRep::Range { min, max };
            }
            _ => return Err("Invalid repetition syntax"),
        }
    } else {
        return Err("Unexpected '{' character");
    }

    Ok(None)
}
