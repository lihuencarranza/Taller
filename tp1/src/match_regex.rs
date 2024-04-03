

use std::cmp::min;

use crate::regex::Regex;
use crate::regex::RegexStep;
use crate::regex::RegexRep;
use crate::regex::RegexValue;
use crate::regex::RegexClass;

#[derive(Debug, PartialEq)]

pub enum MatchState{
    InProgress,
    Matched,
    NotMatched,
}

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
    let mut match_state: MatchState = MatchState::InProgress;
    let mut steps_iter = regex.steps.iter();
    let mut input_chars = chars.chars();


    while match_state == MatchState::InProgress {

        let step = match steps_iter.next() {
            Some(s) => s,
            None => {
                match_state = MatchState::Matched;
                break;
            },
        };

        let actual_char = match input_chars.next(){
            Some(c) => c,
            None => {
                match_state = MatchState::NotMatched;
                break;
            },
        };
        
        match step.rep{
            RegexRep::Any => {
                match &step.val {
                    RegexValue::Literal(c) => {
                        if c != &actual_char {
                            continue;
                        }
                        result.push(actual_char);
                        while let Some(actual_char) = input_chars.next(){
                            if c == &actual_char {
                                result.push(actual_char);
                            }
                        }                        
                    },
                    RegexValue::Wildcard => {
                        if let Some(next_char) = input_chars.next() {
                            result.push(next_char);
                        }
                    },
                    RegexValue::Class(class) => {
                        if !handle_regex_class(&class, actual_char) {
                            continue;
                        }
                        while let Some(next_char) = input_chars.next(){
                            if handle_regex_class(&class, next_char) {
                                result.push(next_char);
                            }
                        }
                    },
                    RegexValue::OneOf(chars) => {
                        if !chars.contains(&actual_char) {
                            continue;
                        }
                        while let Some(next_char) = input_chars.next(){
                            if chars.contains(&next_char) {
                                result.push(next_char);
                            }
                        }
                    },
                    _ => unimplemented!(),
                }
                match_state = MatchState::InProgress;  
                continue;
            },
            RegexRep::Exact(count) =>{
                for _ in 0..count {
                    match &step.val {
                        RegexValue::Literal(c) => {
                            if c != &actual_char {
                                match_state = MatchState::NotMatched;
                                break;
                            }
                        },
                        RegexValue::Wildcard => {},
                        RegexValue::Class(class) => {
                            if !handle_regex_class(&class, actual_char) {
                                match_state = MatchState::NotMatched;
                                break;
                            }
                        },
                        RegexValue::OneOf(content) => {  
                            if !content.contains(&actual_char) {
                                match_state = MatchState::NotMatched;
                                break;
                            } 
                        },
                        RegexValue::Vowel => {
                            if !actual_char.is_ascii_alphabetic() {
                                match_state = MatchState::NotMatched;
                                break;
                            }
                            if !['a', 'e', 'i', 'o', 'u'].contains(&actual_char.to_ascii_lowercase()) {
                                match_state = MatchState::NotMatched;
                                break;
                            }
                        },
                        _ => unimplemented!(),
                    }
                    match_state = MatchState::InProgress;
                    result.push(actual_char);
                    
                }
            },
            RegexRep::Range { min, max } => {
                let mut count = 0;
                while let Some(max) = max {
                    if count >= max {
                        match_state = MatchState::NotMatched};
                        break;
                    }
                    match &step.val {
                        RegexValue::Literal(c) => {
                            if c != &actual_char {
                                match_state = MatchState::NotMatched;
                                break;
                            }
                        },
                        RegexValue::Wildcard => {},
                        RegexValue::Class(class) => {
                            if !handle_regex_class(&class, actual_char) {
                                match_state = MatchState::NotMatched;
                                break;
                            }
                        },
                        RegexValue::OneOf(chars) => {
                            if !chars.contains(&actual_char) {
                                match_state = MatchState::NotMatched;
                                break;
                            }
                        },
                        _ => unimplemented!(),
                    }
                match_state = MatchState::InProgress;
                result.push(actual_char);
                count += 1;
            
                if let Some(min) = min {
                    if count < min {
                        result.clear();
                    }
                }
            },                    
            RegexRep::None => {
                match &step.val {
                    RegexValue::Literal(c) => {
                        if c == &actual_char {
                            match_state = MatchState::NotMatched;
                            break;
                        }
                    },
                    RegexValue::Wildcard => {
                        match_state = MatchState::NotMatched;
                        break;
                    },
                    RegexValue::Class(class) => {
                        if handle_regex_class(&class, actual_char) {
                            match_state = MatchState::NotMatched;
                            break;
                        }
                    },
                    RegexValue::OneOf(chars) => {
                        if chars.contains(&actual_char) {
                            match_state = MatchState::NotMatched;
                            break;
                        }
                    },
                    _ => unimplemented!(),
                }
                match_state = MatchState::InProgress;
            },
        }                  
    }
    if match_state == MatchState::NotMatched{
        if !chars.is_empty(){
            return match_regex(regex, chars[1..].to_string());
        }
        result.clear();
    }

    if result.len() != regex.steps.len(){
        result.clear();
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
    mod t_match_regex{
        use super::*;
    
        mod t_exact{
            use super::*;
             #[test]
            fn t_exact_match_simple_regex() {
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
            fn t_exact_match_one_char(){
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
            #[test]
            fn t_exact_match_simple_regex_fail() {
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
            fn t_exact_match_wildcard(){
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
            fn t_exact_match_class(){
                let regex = Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Class(RegexClass::Digit),
                        },
                    ],
                };
                let s = "1".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "1");

                let s = "a".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");

                let regex = Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Class(RegexClass::Digit),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Class(RegexClass::Digit),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Class(RegexClass::Digit),
                        },
                    ],
                };
                let s = "123".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "123");

                let s = "1a3".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");
            }
        
            #[test]
            fn t_exact_match_vowel(){
                let regex = Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Vowel,
                        },
                    ],
                };
                let s = "a".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "a");

                let s = "b".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");

                let regex = Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Vowel,
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Vowel,
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Vowel,
                        },
                    ],
                };
                let s = "aei".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "aei");

                let s = "aeb".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");
            }

            #[test]
            fn t_exact_on_of(){
                let regex = Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::OneOf(vec!['a', 'b', 'c']),
                        },
                    ],
                };
                let s = "a".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "a");

                let s = "b".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "b");

                let s = "c".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "c");

                let s = "d".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");

                let regex = Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::OneOf(vec!['a', 'b', 'c']),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::OneOf(vec!['a', 'b', 'c']),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::OneOf(vec!['a', 'b', 'c']),
                        },
                    ],
                };
                let s = "abc".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "abc");

                let s = "abd".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");

                let s = "ab".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");
            }
        }

        mod t_range{
        use super::*;

            #[test]
            fn t_range_match_simple_regex() {
                let regex = Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Range { min: Some(1), max: Some(3) },
                            val: RegexValue::Literal('a'),
                        },
                        RegexStep {
                            rep: RegexRep::Range { min: Some(1), max: Some(3) },
                            val: RegexValue::Literal('b'),
                        },
                        RegexStep {
                            rep: RegexRep::Range { min: Some(1), max: Some(3) },
                            val: RegexValue::Literal('c'),
                        },
                    ],
                };
                let s = "abc".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "abc");

                let s = "ab".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");

                let s = "a".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "");

                let s = "abcd".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "abc");

                let s = "aabc".to_string();
                let result = match_regex(&regex, s);
                assert_eq!(result, "aabbc");
            }
    
        }

    }
    
    mod t_comparisson{
        use super::*;
        mod t_compare_exact{
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



        

        /*#[test]
        fn test_compare_asterisk(){
            let regexes = vec![
                Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('a'),
                        },
                        RegexStep {
                            rep: RegexRep::Any,
                            val: RegexValue::Literal('b'),
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
            assert_eq!(result, expected);

            let s = "abbc".to_string();
            let result = compare_regex_with_expression(&regexes, s);
            let expected = vec![
                MatchRegex {
                    matched: "abbc".to_string(),
                    expression: "abbc".to_string(),
                },
            ];

            let s = "ac".to_string();
            let result = compare_regex_with_expression(&regexes, s);
            let expected = vec![
                MatchRegex {
                    matched: "ac".to_string(),
                    expression: "ac".to_string(),
                },
            ];
            assert_eq!(result, expected);
            

            let s = "abdc".to_string();
            let result = compare_regex_with_expression(&regexes, s);
            let expected = vec![];
            assert_eq!(result, expected);
            
        }
        */

        /*#[test]
        fn test_compare_brackets(){
            let regexes = vec![
                Regex {
                    steps: vec![
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('a'),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::OneOf(vec!['b', 'c']),
                        },
                        RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal('c'),
                        },
                    ],
                },
            ];
            let s = "abdcd".to_string();
            let result = compare_regex_with_expression(&regexes, s);
            let expected = vec![
                MatchRegex {
                    matched: "abd".to_string(),
                    expression: "abdcd".to_string(),
                },
            ];
            assert_eq!(result, expected);

            let s = "acd".to_string();
            let result = compare_regex_with_expression(&regexes, s);
            let expected = vec![
                MatchRegex {
                    matched: "acd".to_string(),
                    expression: "acd".to_string(),
                },
            ];
            assert_eq!(result, expected);
        }
        */
    }
}
