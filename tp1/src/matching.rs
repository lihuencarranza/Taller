
use crate::regex;
use crate::regex::Regex;
use crate::regex::RegexRep;
use crate::regex::RegexRestriction;
use crate::regex::RegexStep;
use crate::regex::RegexValue;
use crate::regex::RegexClass;


#[derive(Debug, PartialEq)]
pub enum MatchState{
    Matched,
    NotMatched,
    InProgress,
    EndOfRegex,
    EndOfLine,
}



fn handle_regex_class(class: &RegexClass, c: char) -> bool {
    match class {
        RegexClass::Alpha => c.is_ascii_alphabetic(),
        RegexClass::Alnum => c.is_ascii_alphanumeric(),
        RegexClass::Digit => c.is_digit(10),
        RegexClass::Lower => c.is_ascii_lowercase(),
        RegexClass::Upper => c.is_ascii_uppercase(),
        RegexClass::Punct => c.is_ascii_punctuation(),
        RegexClass::Space => c.is_ascii_whitespace(),
    }
}

fn is_end_of_line(backtracking: &Option<Vec<regex::RegexRestriction>>)->bool{
    if let Some(restrictions) = backtracking{
        for restriction in restrictions{
            match restriction{
                RegexRestriction::EndOfLine => return true,
                _ => continue,
            }
        }
    }

    false
}

fn is_start_of_line(backtracking: &Option<Vec<regex::RegexRestriction>>)->bool{
    if let Some(restrictions) = backtracking{
        for restriction in restrictions{
            match restriction{
                RegexRestriction::StartOfLine => return true,
                _ => continue,
            }
        }
    }

    false
}



fn handle_none_case(step: &RegexStep, actual_char: char, match_state: &mut MatchState) -> bool {
    match &step.val {
        RegexValue::Literal(c) => {
            if c == &actual_char {
                *match_state = MatchState::NotMatched;
                return false;
            }
        },
        RegexValue::Wildcard => {
            *match_state = MatchState::NotMatched;
            return false;
        },
        RegexValue::Class(class) => {
            if handle_regex_class(&class, actual_char) {
                *match_state = MatchState::NotMatched;
                return false;
            }
        },
        RegexValue::OneOf(chars) => {
            if chars.contains(&actual_char) {
                *match_state = MatchState::NotMatched;
                return false;
            }
        },
        _ => unimplemented!(),
    }
    *match_state = MatchState::InProgress;
    true
}

fn handle_exact_case(step: &RegexStep, mut actual_char: char, match_state: &mut MatchState, count: usize, input_chars: &mut std::str::Chars, result: &mut String) -> char {
    for index in 0..count {
        match &step.val {
            RegexValue::Literal(c) => {
                if c != &actual_char {
                    *match_state = MatchState::NotMatched;
                    break;
                }
            },
            RegexValue::Wildcard => {},
            RegexValue::Class(class) => {
                if !handle_regex_class(&class, actual_char) {
                    *match_state = MatchState::NotMatched;
                    break;
                }
            },
            RegexValue::OneOf(content) => {  
                if !content.contains(&actual_char) {
                    *match_state = MatchState::NotMatched;
                    break;
                } 
            },
            _ => unimplemented!(),
        }
        
        *match_state = MatchState::InProgress;
        result.push(actual_char);
        if index < count - 1 {
            actual_char = match input_chars.next() {
                Some(c) => c,
                None => { *match_state = MatchState::EndOfLine; break; }
            };
        }
    }
    actual_char
}


fn compare_regex_with_expression(regex: &Regex, word: &String)-> String{
    let mut result = String::new();
    let mut match_state: MatchState = MatchState::InProgress;
    let mut steps_iter = regex.steps.iter();
    let mut input_chars = word.chars();
 
    while match_state == MatchState::InProgress {
        
        let step = match steps_iter.next() {
            Some(s) => s,
            None => {match_state = MatchState::EndOfRegex; break;},
        };
        
        let mut actual_char = match input_chars.next(){
            Some(c) => c,
            None => {match_state = MatchState::EndOfLine; break;}
        };

        match step.rep{
            RegexRep::Exact(count) =>{
                actual_char = handle_exact_case(step, actual_char, &mut match_state, count, &mut input_chars, &mut result);
            
            },
            RegexRep::Range { mut min, max } => {
                match min{
                    Some(min_count) => {
                        actual_char = handle_exact_case(step, actual_char, &mut match_state, min_count, &mut input_chars, &mut result);
                    },
                    None => {},
                }

                match max{
                    Some(max_count) => todo!(),
                    None => {},
                }




                
                
            },
            RegexRep::None => {
                if handle_none_case(step, actual_char, &mut match_state) {
                    result.push(actual_char);
                    match_state = MatchState::InProgress;
                }
            },
        
        }                         
    }

    if match_state == MatchState::EndOfRegex{
        if is_end_of_line(&regex.backtracking) && input_chars.next().is_some(){
            return compare_regex_with_expression(regex, &word[1..].to_string());
        } 
        return result;
    }


    if !is_start_of_line(&regex.backtracking) && input_chars.next().is_some(){
        return compare_regex_with_expression(regex, &word[1..].to_string());
    }

    "".to_string()
}



pub fn compare_regexes_with_expression(regexes: &Vec<Regex>, s: String)-> Result<String, &'static str>{

    for regex in regexes{
        let result = compare_regex_with_expression(regex, &s);
        if !result.is_empty(){
            return Ok(s);
        }
    }
    
    Err("No match found")
    
}



#[cfg(test)]
mod tests {
    use super::*;

    mod exact{
        use super::*;  

        mod literal{
            use super::*;
            
            #[test]
            fn test_1(){
                let regex = regex::Regex::new("a").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);

            }

            #[test]
            fn test_2(){
                let regex = regex::Regex::new(".").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_3(){
                let regex = regex::Regex::new("a").unwrap();
                let word = "b".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_4(){
                let regex = regex::Regex::new("a").unwrap();
                let word = "ab".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
            }

            #[test]
            fn test_5(){
                let regex = regex::Regex::new("a").unwrap();
                let word = "ba".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
            }

            #[test]
            fn test_6(){
                let regex = regex::Regex::new("a").unwrap();
                let word = "bab".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
            }

        }

        mod wildcard{
            use super::*;

            #[test]
            fn test_1(){
                let regex = regex::Regex::new(".").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_2(){
                let regex = regex::Regex::new(".").unwrap();
                let word = "ab".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
            }

            #[test]
            fn test_3(){
                let regex = regex::Regex::new(".").unwrap();
                let word = "ba".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "b".to_string());
            }

            #[test]
            fn test_4(){
                let regex = regex::Regex::new(".").unwrap();
                let word = "bab".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "b".to_string());
            }
        }

        mod classes{
            use super::*;

            #[test]
            fn test_alpha(){
                let regex = regex::Regex::new("[[:alpha:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_alpha_f(){
                let regex = regex::Regex::new("[[:alpha:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_alnum(){
                let regex = regex::Regex::new("[[:alnum:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_alnum_f(){
                let regex = regex::Regex::new("[[:alnum:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_digit(){
                let regex = regex::Regex::new("[[:digit:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_digit_f(){
                let regex = regex::Regex::new("[[:digit:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_lower(){
                let regex = regex::Regex::new("[[:lower:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_lower_f(){
                let regex = regex::Regex::new("[[:lower:]]").unwrap();
                let word = "A".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_upper(){
                let regex = regex::Regex::new("[[:upper:]]").unwrap();
                let word = "A".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_upper_f(){
                let regex = regex::Regex::new("[[:upper:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_punct(){
                let regex = regex::Regex::new("[[:punct:]]").unwrap();
                let word = "!".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_punct_f(){
                let regex = regex::Regex::new("[[:punct:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_space(){
                let regex = regex::Regex::new("[[:space:]]").unwrap();
                let word = " ".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_space_f(){
                let regex = regex::Regex::new("[[:space:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

           

           

           
        }
    
        mod oneof{
            use super::*;

            #[test]
            fn test_1(){
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_2(){
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "b".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_3(){
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "c".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_4(){
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "d".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_5(){
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "ab".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
            }

            #[test]
            fn test_6(){
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "ba".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "b".to_string());
            }

            #[test]
            fn test_7(){
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "bab".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "b".to_string());
            }
        }
    
    }

    mod none{
        use super::*;

        #[test]
        fn vocal(){
            let regex = regex::Regex::new("[^aeiou]").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "c".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn literals(){
            let regex = regex::Regex::new("[^abc]").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "d".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        mod classes{
            use super::*;
            #[test]
            fn alpha(){
                let regex = regex::Regex::new("[^[:alpha:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "1".to_string());
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "%".to_string());
            }

            #[test]
            fn alnum(){
                let regex = regex::Regex::new("[^[:alnum:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "%".to_string());
            }

            #[test]
            fn digit(){
                let regex = regex::Regex::new("[^[:digit:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "%".to_string());
            }

            #[test]
            fn lower(){
                let regex = regex::Regex::new("[^[:lower:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "A".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "A".to_string());
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "%".to_string());
            }

            #[test]
            fn upper(){
                let regex = regex::Regex::new("[^[:upper:]]").unwrap();
                let word = "A".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "%".to_string());
            }

            #[test]
            fn punct(){
                let regex = regex::Regex::new("[^[:punct:]]").unwrap();
                let word = "!".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn space(){
                let regex = regex::Regex::new("[^[:space:]]").unwrap();
                let word = " ".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "%".to_string());
            }
     
        }
        

    }
    
    mod range{
        use super::*;

            
        #[test]
        fn test_1(){
            let regex = regex::Regex::new("a{2}").unwrap();
            let word = "aa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn test_2(){
            let regex = regex::Regex::new("a{2}").unwrap();
            let word = "aaa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "aa".to_string());
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }

        #[test]
        fn test_3(){
            let regex = regex::Regex::new("a{2,}").unwrap();
            let word = "aa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "aaa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "aa".to_string());
        }

        /*#[test]
        fn test_4(){
            let regex = regex::Regex::new("a?").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }*/
        
    }

    mod start_of_line{
        use super::*;

        #[test]
        fn test_1(){
            let regex = regex::Regex::new("^a").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn test_2(){
            let regex = regex::Regex::new("^a").unwrap();
            let word = "ba".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }

        #[test]
        fn test_3(){
            let regex = regex::Regex::new("^[aeiou]").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "ba".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "b".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }
    }
  
    mod end_of_line{
        use super::*;

        #[test]
        fn test_1(){
            let regex = regex::Regex::new("a$").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn test_2(){
            let regex = regex::Regex::new("a$").unwrap();
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }

        #[test]
        fn test_3(){
            let regex = regex::Regex::new("[aeiou]$").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "ba".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "a".to_string());
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "b".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }
    }
}