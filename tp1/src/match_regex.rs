
use crate::regex::Regex;
use crate::regex::RegexStep;
use crate::regex::RegexRep;
use crate::regex::RegexValue;
use crate::regex::RegexClass;

#[derive(Debug, PartialEq)]
pub struct MatchRegex {
    pub matched: String,
    pub expression: String,
}

fn handle_regex_class(class: &RegexClass, c: char) -> bool {   
    match class {
        RegexClass::Digit => {
            if c.is_digit(10) {
                return true;
            }
        }
        RegexClass::Space => {
            if c.is_whitespace() {
                return true;
            }
        }
        RegexClass::Lower => {
            if c.is_lowercase() {
                return true;
            }
        }
        RegexClass::Upper => {
            if c.is_uppercase() {
                return true;
            }
        }
        RegexClass::Alpha => {
            if c.is_alphabetic() {
                return true;
            }
        }
        RegexClass::Alnum => {
            if c.is_alphanumeric() {
                return true;
            }
        }
        RegexClass::Punct => {
            if c.is_ascii_punctuation() {
                return true;
            }
        }
    }
    false
}

fn match_regex(regex: &Regex, chars: String) -> String {
    let mut result = String::new();
    let mut match_state = false;

    for step in regex.steps.iter() {
        let mut input_chars = chars.chars();
        for actual_char in input_chars{
            match step.rep{
                RegexRep::Any => {
                    match &step.val {
                        RegexValue::Literal(c) => {
                            if c == &actual_char {
                                result.push(actual_char);
                            }
                        },
                        RegexValue::Wildcard => {
                            result.push(actual_char);
                        },
                        RegexValue::Class(class) => {
                            if handle_regex_class(&class, actual_char) {
                                result.push(actual_char);
                            }
                        },
                        RegexValue::OneOf(chars) => {
                            if chars.contains(&actual_char) {
                                result.push(actual_char);
                            }
                        },
                        _ => unimplemented!(),
                    }
                    
                },
                RegexRep::Exact(count) =>{
                    for _ in 0..count {
                        match &step.val {
                            RegexValue::Literal(c) => {
                                if *c == actual_char {
                                    result.push(actual_char);
                                }
                            },
                            RegexValue::Wildcard => {
                                result.push(actual_char);
                            },
                            RegexValue::Class(class) => {
                                if handle_regex_class(&class, actual_char) {
                                    result.push(actual_char);
                                }
                            },
                            RegexValue::OneOf(content) => {
                                for p in content.iter() {
                                    if p == &actual_char {
                                        result.push(actual_char);
                                    }
                                }
                            },
                            _ => unimplemented!(),
                        }
                    }
                },
                RegexRep::Range { min, max } => {
                    let mut count = 0;
                    if let Some(max) = max {
                        if count >= max {
                            break;
                        }
                    }
                    match &step.val {
                        RegexValue::Literal(c) => {
                            if *c == actual_char {
                                result.push(actual_char);
                                count += 1;
                            } else {
                                break;
                            }
                        },
                        RegexValue::Wildcard => {
                            result.push(actual_char);
                            count += 1;
                        },
                        RegexValue::Class(class) => {
                            if handle_regex_class(&class, actual_char) {
                                result.push(actual_char);
                                count += 1;
                            } else {
                                break;
                            }
                        },
                        RegexValue::OneOf(chars) => {
                            if chars.contains(&actual_char) {
                                result.push(actual_char);
                                count += 1;
                            } else {
                                break;
                            }
                        },
                        _ => unimplemented!(),
                    }
                
                    if let Some(min) = min {
                        if count < min {
                            // If we haven't matched the minimum number of repetitions, the match fails
                            result.clear();
                        }
                    }
                },                    
                RegexRep::None => {
                    match &step.val {
                        RegexValue::Literal(c) => {
                            if *c == actual_char {
                                result.clear();
                                break;
                            }
                        },
                        RegexValue::Wildcard => {
                            result.clear();
                            break;
                        },
                        RegexValue::Class(class) => {
                            if handle_regex_class(&class, actual_char) {
                                result.clear();
                                break;
                            }
                        },
                        RegexValue::OneOf(chars) => {
                            if chars.contains(&actual_char) {
                                result.clear();
                                break;
                            }
                        },
                        _ => unimplemented!(),
                    }
                },
            }
        }
    }



    if result.is_empty() {
        return match_regex(regex,chars[1..].to_string());
    }      

    result
}

pub fn compare_regex_with_expression(regexes: &Vec<Regex>, s: String)-> Vec<MatchRegex>{
    let mut coincidence: Vec<String> = Vec::new();
    for r in regexes.iter() {
        let matched = match_regex(r, s.clone());
        if !matched.is_empty() {
            coincidence.push(matched);
            break;
        }
    }

    let mut result: Vec<MatchRegex> = Vec::new();
    if coincidence.is_empty() {
        return result;
    }
    for c in coincidence.iter() {
        result.push(MatchRegex {matched: c.to_string(), expression: s.to_string()});
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_match_regex{
        use super::*;

        #[test]
        fn test_match_simple_regex() {
            let regex = Regex {
                steps: vec![
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('a'),
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('b'),
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('c'),
                    },
                ],
            };
            let s = "abc".to_string();
            let result = match_regex(&regex, s);
            assert_eq!(result, "abc");
        }

        #[test]
        fn test_match_simple_regex_fail() {
            let regex = Regex {
                steps: vec![
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('a'),
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('b'),
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('c'),
                    },
                ],
            };
            let s = "def".to_string();
            let result = match_regex(&regex, s);
            assert_eq!(result, "");
        }

        #[test]
        fn test_match_wildcard(){
            let regex = Regex {
                steps: vec![
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Wildcard,
                    },
                ],
            };
            let s = "a".to_string();
            let result = match_regex(&regex, s);
            assert_eq!(result, "a");


            let regex = Regex {
                steps: vec![
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Wildcard,
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('b'),
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('c'),
                    },
                ],
            };
            let s = "abc".to_string();
            let result = match_regex(&regex, s);
            assert_eq!(result, "abc");

            let regex = Regex {
                steps: vec![
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('a'),
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Wildcard,
                    },
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('c'),
                    },
                ],
            };
            let s = "abc".to_string();
            let result = match_regex(&regex, s);
            assert_eq!(result, "abc");
        }
    
        #[test]
        fn test_match_any(){
            let regex = Regex {
                steps: vec![
                    RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexValue::Literal('b'),
                    },
                ],
            };
            let s = "a".to_string();
            let result = match_regex(&regex, s);
            assert_ne!(result, "a");

            let s = "abc".to_string();
            let result = match_regex(&regex, s);
            assert_eq!(result, "b");

            let s = "abcd".to_string();
            let result = match_regex(&regex, s);
            assert_eq!(result, "b");
        }
    

    }
    
    mod test_comparisson{
        use super::*;

        #[test]
        fn test_compare_regex_with_expression() {
            let regexes = vec![
                Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('a'),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('b'),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('c'),
                        },
                    ],
                },
                Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('a'),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Wildcard,
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('c'),
                        },
                    ],
                },
            ];
            let s = "abc".to_string();
            let result = compare_regex_with_expression(&regexes, s);
            let expected = vec![
                MatchRegex {
                    matched: "abc".to_string(),
                    expression: "abc".to_string(),
                },
            ];
        }

        #[test]
        fn test_compare_regex_with_expression_2() {
            let regexes = vec![
                Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('b'),
                        },
                        
                    ],
                }, 
            ];
            let s = "abc".to_string();
            let expected = vec![
            MatchRegex {
                matched: "b".to_string(),
                expression: "abc".to_string(),
            }];
            let result = compare_regex_with_expression(&regexes, s);
            assert_eq!(result, expected);
        }
    }
}
