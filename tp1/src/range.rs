use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use std::str::Chars;

pub fn handle_any(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range { min: None, max: None };
    } else {
        return Err("Unexpected '*' character");
    }
    Ok(None)
}

pub fn handle_zero_or_one(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range { min: None, max: Some(1) };
    } else {
        return Err("Unexpected '?' character");
    }
    Ok(None)
}



pub fn handle_exact_plus(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, &'static str> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range { min: Some(1), max: None };
    } else {
        return Err("Unexpected '+' character");
    }
    Ok(None)
}

pub fn handle_range(
    chars_iter: &mut Chars,
    steps: &mut Vec<RegexStep>,
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
