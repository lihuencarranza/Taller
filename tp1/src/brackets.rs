use crate::regex::RegexClass;
use crate::regex::RegexRep;
use crate::regex::RegexStep;
use crate::regex::RegexValue;
use std::str::Chars;

fn handle_metachar(n: String) -> Result<RegexClass, &'static str> {
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

fn handle_random_string(n: String) -> Result<Vec<char>, &'static str> {
    Ok(n.chars().collect())
}

fn handle_content(chars: &mut Chars) -> Result<RegexValue, &'static str> {
    let n = chars.as_str().to_string();
    let chars_vec: Vec<char> = n.chars().collect();
    let value: RegexValue;
    match n.as_str() {
        "aeiou" => {
            value = RegexValue::Vowel;
        }
        "0-9" => {
            value = RegexValue::Class(RegexClass::Digit);
        }
        "a-z" => {
            value = RegexValue::Class(RegexClass::Lower);
        }
        "A-Z" => {
            value = RegexValue::Class(RegexClass::Upper);
        }
        _ => {
            if chars_vec[0] == ':' {
                value = RegexValue::Class(handle_metachar(n)?);
            } else {
                value = RegexValue::OneOf(handle_random_string(n)?);
            }
        }
    }

    Ok(value)
}

fn handle_not(chars: &mut Chars) -> Result<Option<RegexStep>, &'static str> {
    let value = handle_content(chars)?;
    Ok(Some(RegexStep {
        rep: RegexRep::None,
        val: value,
    }))
}

fn handle_optional(chars: &mut Chars) -> Result<Option<RegexStep>, &'static str> {
    let value = handle_content(chars)?;
    Ok(Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: value,
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
                    if n.is_empty() || (*flag && n.len() == 1 && n.chars().next().unwrap() == '^') {
                        return Err("Empty brackets or invalid caret usage");
                    }
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
    if n.is_empty() {
        return Err("Empty brackets");
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

    mod valid_basics {
        use super::*;

        #[test]
        fn brackets_vowel() {
            let mut s = "[aeiou]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_not_vowel() {
            let mut s = "[^aeiou]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_class_digit() {
            let mut s = "[0-9]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_class_lower() {
            let mut s = "[a-z]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_class_upper() {
            let mut s = "[A-Z]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_metachar_alpha() {
            let mut s = "[:alpha:]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_metachar_alnum() {
            let mut s = "[:alnum:]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }
    }

    mod not_valid {
        use super::*;

        #[test]
        fn brackets_empty() {
            let mut s = "[]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn brackets_not_empty() {
            let mut s = "[^]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn brackets_class_alpha_invalid() {
            let mut s = "[:alpha]".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_err(), true);
        }
    }

    mod more_tests {
        use super::*;

        #[test]
        fn brackets_vowel_1() {
            let mut s = "[aeiou]a".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_empty_1() {
            let mut s = "[]a".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn brackets_not_empty_1() {
            let mut s = "[^]a".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn brackets_not_vowel_1() {
            let mut s = "[^aeiou]a".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_vowel_2() {
            let mut s = "[aeiou]a".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn brackets_empty_2() {
            let mut s = "[]a".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn brackets_not_empty_2() {
            let mut s = "[^]a".chars();
            let result = handle_brackets(&mut s);
            assert_eq!(result.is_err(), true);
        }
    }
}
