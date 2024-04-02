use crate::regex::RegexRep;
use crate::regex::RegexStep;
use crate::regex::RegexValue;
use std::str::Chars;
use std::vec;

fn handle_vowel() -> Vec<char> {
    vec!['a', 'e', 'i', 'o', 'u']
}

fn handle_digit() -> Vec<char> {
    vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
}

fn handle_lower() -> Vec<char> {
    vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
}

fn handle_upper() -> Vec<char> {
    vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
}

fn handle_alpha() -> Vec<char> {
    let mut alpha = handle_lower();
    let upper = handle_upper();
    alpha.extend(upper);
    alpha
}

fn handle_punct() -> Vec<char> {
    vec![
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ]
}

fn handle_space() -> Vec<char> {
    vec![' ', '\t', '\n', '\r', '\x0c', '\x0b']
}

fn handle_alnum() -> Vec<char> {
    let mut alnum = handle_lower();
    let upper = handle_upper();
    alnum.extend(upper);
    let digit = handle_digit();
    alnum.extend(digit);
    alnum
}

fn handle_metachar(n: String) -> Result<Vec<char>, &'static str> {
    Ok(match n.as_str() {
        ":alpha:" => handle_alpha(),
        ":alnum:" => handle_alnum(),
        ":upper:" => handle_upper(),
        ":lower:" => handle_lower(),
        ":space:" => handle_space(),
        ":punct:" => handle_punct(),
        ":digit:" => handle_digit(),
        _ => return Err("Invalid metacharacter"),
    })
}

fn handle_random_string(n: String) -> Result<Vec<char>, &'static str> {
    Ok(n.chars().collect())
}

fn handle_content(chars: &mut Chars) -> Result<Vec<char>, &'static str> {
    let n = chars.as_str().to_string();
    let chars_vec: Vec<char> = n.chars().collect();
    match n.as_str() {
        "aeiou" => return Ok(handle_vowel()),
        "0-9" => {
            return Ok(handle_digit());
        }
        "a-z" => {
            return Ok(handle_lower());
        }
        "A-Z" => {
            return Ok(handle_upper());
        }
        _ => {
            if chars_vec[0] == ':' {
                return handle_metachar(n);
            } else {
                return handle_random_string(n);
            }
        }
    }
}

fn handle_not(chars: &mut Chars) -> Result<Option<RegexStep>, &'static str> {
    let content = handle_content(chars)?;
    Ok(Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexValue::Not(content),
    }))
}

fn handle_optional(chars: &mut Chars) -> Result<Option<RegexStep>, &'static str> {
    let content = handle_content(chars)?;
    Ok(Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexValue::Optional(content),
    }))
}

fn process_inside_brackets(
    chars_iter: &mut Chars,
    n: &mut String,
    flag: &mut bool,
    inside_brackets: &mut bool,
) -> Result<(), &'static str> {
    while let Some(c) = chars_iter.next() {
        match c {
            '[' => {
                *inside_brackets = true;
                continue;
            }
            ']' => {
                if *inside_brackets {
                    *inside_brackets = false;
                    continue;
                } else {
                    break;
                }
            }
            '^' => {
                *flag = true;
                continue;
            }
            _ => n.push(c),
        }
    }
    Ok(())
}

pub fn handle_brackets(chars_iter: &mut Chars) -> Result<Option<RegexStep>, &'static str> {
    let mut n = String::new();
    let mut flag = false;
    let mut inside_brackets = false;

    process_inside_brackets(chars_iter, &mut n, &mut flag, &mut inside_brackets)?;

    if n.is_empty() {
        return Err("Empty brackets");
    }
    if flag {
        handle_not(&mut n.chars())
    } else {
        handle_optional(&mut n.chars())
    }
}

#[cfg(test)]
mod brackets_tests {
    use super::*;

    #[test]
    fn test_handle_vowel() {
        let vowels = handle_vowel();
        assert_eq!(vowels, vec!['a', 'e', 'i', 'o', 'u']);
    }

    #[test]
    fn test_handle_digit() {
        let digits = handle_digit();
        assert_eq!(
            digits,
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }

    #[test]
    fn test_handle_lower() {
        let lower = handle_lower();
        assert_eq!(
            lower,
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }

    #[test]
    fn test_handle_upper() {
        let upper = handle_upper();
        assert_eq!(
            upper,
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_handle_alnum() {
        let alnum = handle_alnum();
        let lower = handle_lower();
        let upper = handle_upper();
        let digit = handle_digit();
        let mut expected = lower;
        expected.extend(upper);
        expected.extend(digit);
        assert_eq!(alnum, expected);
    }

    #[test]
    fn test_handle_punct() {
        let punct = handle_punct();
        assert_eq!(
            punct,
            vec![
                '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
                ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'
            ]
        );
    }

    #[test]
    fn test_handle_space() {
        let space = handle_space();
        assert_eq!(space, vec![' ', '\t', '\n', '\r', '\x0c', '\x0b']);
    }

    #[test]
    fn test_handle_not() {
        let mut chars = "aeiou".chars();
        let result = handle_not(&mut chars);
        let expected = vec!['a', 'e', 'i', 'o', 'u'];
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Not(expected));
    }

    #[test]
    fn test_handle_optional() {
        let mut chars = "aeiou".chars();
        let result = handle_optional(&mut chars);
        let expected = vec!['a', 'e', 'i', 'o', 'u'];
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Optional(expected));

        let mut chars = "nam".chars();
        let result = handle_optional(&mut chars);
        let expected = vec!['n', 'a', 'm'];
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Optional(expected));
    }

    #[test]
    fn test_handle_brackets() {
        let mut chars = "[aeiou]".chars();
        let result = handle_brackets(&mut chars);
        let expected = vec!['a', 'e', 'i', 'o', 'u'];
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Optional(expected));

        let mut chars = "[^aeiou]".chars();
        let result = handle_brackets(&mut chars);
        let expected = vec!['a', 'e', 'i', 'o', 'u'];
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Not(expected));

        let mut chars = "[0-9]".chars();
        let result = handle_brackets(&mut chars);
        let expected = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Optional(expected));

        let mut chars = "[[:alpha:]]".chars();
        let result = handle_brackets(&mut chars);
        let expected = handle_alpha();
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Optional(expected));

        let mut chars = "[[:alnum:]]".chars();
        let result = handle_brackets(&mut chars);
        let expected = handle_alnum();
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Optional(expected));
    }

    #[test]
    fn random_tests() {
        let mut chars = "[[:alpha:]]".chars();
        let result = handle_brackets(&mut chars);
        let expected = handle_alpha();
        assert_eq!(result.unwrap().unwrap().val, RegexValue::Optional(expected));
        assert_eq!(chars.as_str(), "");
    }
}
