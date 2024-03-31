
#[derive(Debug,PartialEq)]
pub enum Match {
    NotStarted,
    InProgress,
    Completed,
    Canceled,
}

#[derive(Debug, PartialEq)]
enum RepType{
    ZeroOrMore,// "*"
    //OneOrMore, // "+"
    ZeroOrOne, // "?"
    //ExactQuantifier, // {n} 
    //AtLeastQuantifier, //{n,} 
    //UpToQuantifier, // {,m}
    //RangeQuantifier, // {n,m}
}

#[derive(Debug, PartialEq)]
enum RegexClass{
    Period,
    Repetition(RepType),
    //Other,
}

#[derive(Debug, PartialEq)]
enum RegexValue{
    Literal(char),
    Wildcard(RegexClass), 
}

#[derive(Debug, PartialEq)]
enum RegexRep{
    Any,
    Exact(usize), //{n}
    Range{
        min: Option<usize>,
        max: Option<usize>,
    },
}

#[derive(Debug, PartialEq)]
pub struct RegexStep{
    val: RegexValue,
    rep: RegexRep,
    prev_char: Option<char>,
}

#[derive(Debug, PartialEq)]
pub struct Regex {
	steps: Vec<RegexStep>
}

impl Regex{

	pub fn new(expression: &str) -> Result<Self, &str> {
        if expression.is_empty() {
            return Err("Empty expression");
        }

        let mut steps: Vec<RegexStep> = vec![];

        let mut chars_iter = expression.chars();

        let mut last_char: Option<char> = None;

        while let Some(c) = chars_iter.next(){
            
            let step = match c {
                '?' => 
                    Some(RegexStep{
                    rep: RegexRep::Exact(1),
                    val: RegexValue::Wildcard(RegexClass::Repetition(RepType::ZeroOrOne)),
                    prev_char: last_char,
                }),
                '*' => 
                    Some(RegexStep{
                    rep: RegexRep::Any,
                    val: RegexValue::Wildcard(RegexClass::Repetition(RepType::ZeroOrMore)),
                    prev_char: last_char,
                }),
                '.' => Some(RegexStep{
                    rep: RegexRep::Exact(1), 
                    val: RegexValue::Wildcard(RegexClass::Period),
                    prev_char: last_char,
                }),
                'a'..='z' => Some(RegexStep{
                    rep: RegexRep::Exact(1),
                    val: RegexValue::Literal(c),
                    prev_char: last_char,
                }),                
                
                _ => return Err("Invalid character in expression"),

            };
            
            if let Some(p) = step {
                steps.push(p);
            }

            last_char = Some(c);
        }

        if !is_regex_step_valid(&steps){
            return Err("Invalid expression");
        }

    	Ok(Regex{steps})
	}
}

pub fn is_regex_step_valid(steps: &Vec<RegexStep>) -> bool{
    let mut iter = steps.iter();

    if let Some(step) = iter.next(){
        if let RegexValue::Wildcard(RegexClass::Repetition(_)) = step.val {
            if step.prev_char.is_some(){
                return false;
            }
        };
    }
    true
}


pub fn check_word_with_regex(regex: &Regex, word: &str) -> bool {
    let word_chars: Vec<char> = word.chars().collect();
    let mut word_index = 0;
    let mut step_iter = regex.steps.iter();
    let mut matching = Match::InProgress;
    let mut step = step_iter.next();

    while matching == Match::InProgress && &word_index < &word_chars.len() {
        match step {
            Some(s) => match s.val {
                RegexValue::Literal(l) => {
                    if word_chars[word_index] != l {
                        matching = Match::NotStarted;
                    } else {
                        word_index += 1;
                    }
                },
                RegexValue::Wildcard(RegexClass::Period) => {
                    word_index += 1;
                },
                RegexValue::Wildcard(RegexClass::Repetition(_)) =>todo!(),
            },
            None => break,
        }
        if matching == Match::InProgress {
            step = step_iter.next();
        } else {
            break;
        }
        
    }
    if matching == Match::InProgress && step.is_none()  {
        true
    }else if word_index < word_chars.len() {
        return check_word_with_regex(regex, &word[1..]);
    } else {
        false
    }
   
}






/*pub fn check_word_with_regex(regex: &Regex, word: &str) -> bool {
    let mut word_iter = word.chars().peekable();
    let mut steps_iter = regex.steps.iter();
    let mut matching = Match::NotStarted;
    let mut index = 0;

    while let Some(c) = word_iter.next() {
        if let Some(step) = steps_iter.next() {
            matching = match_step_value(Some(step), &c, &mut word.chars().collect::<Vec<char>>(), &mut steps_iter, &mut index);
            if matching == Match::NotStarted {
                steps_iter = regex.steps.iter();
            }
        } else{
            break;
        }
    }

    if matching == Match::InProgress && steps_iter.next().is_none() {
        matching = Match::Completed;
    }

    if matching == Match::Completed {
        true
    } else if !word.is_empty() {
        check_word_with_regex(regex, &word[1..])
    } else {
        false
    }
}

pub fn match_step_value(step: Option<&RegexStep>, c: &char, word_vec: &mut Vec<char>, steps_iter: &mut std::slice::Iter<'_, RegexStep>, index: &mut i32) -> Match {

    if step.is_none() {
        return Match::NotStarted;
    }
    
    match step {
        Some(s)=> match &step.unwrap().val{
            RegexValue::Wildcard(RegexClass::Period) => {
            *index += 1;
            Match::InProgress
            },
            RegexValue::Wildcard(RegexClass::Repetition(rep_type)) => {
                match rep_type {
                    RepType::ZeroOrOne => {
                        let i = (*index as i32) - 1;
                        if word_vec[i as usize] == step.unwrap().prev_char.unwrap() {
                            *index += 1;
                        }
                        Match::InProgress
                    },
                    RepType::ZeroOrMore => todo!(),
                    //RepType::OneOrMore => todo!(),
                    // RepType::ExactQuantifier => todo!(),
                    //RepType::AtLeastQuantifier => todo!(),
                    //RepType::UpToQuantifier => todo!(),
                    //RepType::RangeQuantifier => todo!(),
                    // Handle other repetition types...
                }
                
            },
            RegexValue::Literal(l) => {
                // For a literal, check if the current character is the same as the literal.
            if l == c {
                    *index += 1;
                    Match::InProgress
                }else{
                    *index += 1;
                    Match::NotStarted
                }
                    
            },
            _ => Match::NotStarted,
        }
        None => todo!(),
        
    }
}*/






#[cfg(test)]
mod tests {
    use super::*;
    mod regex_tests{
        use super::*;
        #[test]
        fn test_regex_new() {
            let regex = Regex::new("hola").unwrap();
            assert_eq!(regex.steps.len(), 4);
        }

        #[test]
        fn test_regex_new_empty() {
            let regex = Regex::new("").unwrap_err();
            assert_eq!(regex, "Empty expression");
        }

    }

    mod regex_step_tests{
        use super::*;
        #[test]
        fn test_regex_step_new() {
            let step = RegexStep{
                rep: RegexRep::Exact(1),
                val: RegexValue::Literal('a'),
                prev_char: None,
            };
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.val, RegexValue::Literal('a'));
            assert_eq!(step.prev_char, None);
        }        
    }

    /*mod regex_value_tests{
        use super::*;
        #[test]
        fn test_literal() {
            let regex = Regex::new("hola").unwrap();
            let step = &regex.steps[0];
            assert_eq!(step.val, RegexValue::Literal('h'));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, None);
            assert_ne!(step.val, RegexValue::Wildcard(RegexClass::Period));
        }

        #[test]
        fn test_wildcard_simple_period() {
            let regex = Regex::new(".").unwrap();
            let step = &regex.steps[0];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, None);
        }    

        #[test]
        fn test_wildcard_period_1() {
            let regex = Regex::new(".a").unwrap();
            let step = &regex.steps[0];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, None); 
            let step = &regex.steps[1];
            assert_eq!(step.val, RegexValue::Literal('a'));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, Some('.'));       
        }

        #[test]
        fn test_wildcard_period_2() {
            let regex = Regex::new("..a").unwrap();
            let step = &regex.steps[1];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, Some('.')); 
            
            let step = &regex.steps[2];
            assert_eq!(step.val, RegexValue::Literal('a'));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, Some('.')); 
            
        }

        #[test]
        fn test_wildcard_period_3() {
            let regex = Regex::new("b.").unwrap();
            let step = &regex.steps[1];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, Some('b')); 
        }

        #[test]
        fn test_wildcard_period_4() {
            let regex = Regex::new("b..").unwrap();
            let step = &regex.steps[2];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.prev_char, Some('.')); 
        }

    }

    
    mod test_match_step_value{
        use super::*;
        #[test]
        fn test_match_step_value_literal() {
            let regex = Regex::new("hola").unwrap();
            let step = &regex.steps[0];
            let c = 'h';
            let mut word_vec = vec!['h', 'o', 'l', 'a'];
            let mut steps_iter = regex.steps.iter();
            let mut index = 0;
            let matching = match_step_value(Some(step), &c, &mut word_vec, &mut steps_iter, &mut index);
            assert_eq!(matching, Match::InProgress);
        }

        #[test]
        fn test_match_step_value_literal_fail() {
            let regex = Regex::new("hola").unwrap();
            let step = &regex.steps[0];
            let c = 'a';
            let mut word_vec = vec!['h', 'o', 'l', 'a'];
            let mut steps_iter = regex.steps.iter();
            let mut index = 0;
            let matching = match_step_value(Some(step), &c, &mut word_vec, &mut steps_iter, &mut index);
            assert_eq!(matching, Match::NotStarted);
        }

        #[test]
        fn test_match_step_value_period() {
            let regex = Regex::new(".").unwrap();
            let step = &regex.steps[0];
            let c = 'h';
            let mut word_vec = vec!['h', 'o', 'l', 'a'];
            let mut steps_iter = regex.steps.iter();
            let mut index = 0;
            let matching = match_step_value(Some(step), &c, &mut word_vec, &mut steps_iter, &mut index);
            assert_eq!(matching, Match::InProgress);
        }

        #[test]
        fn test_match_step_value_period_1() {
            let regex = Regex::new(".a").unwrap();
            let step = &regex.steps[0];
            let c = 'h';
            let mut word_vec = vec!['h', 'o', 'l', 'a'];
            let mut steps_iter = regex.steps.iter();
            let mut index = 0;
            let matching = match_step_value(Some(step), &c, &mut word_vec, &mut steps_iter, &mut index);
            assert_eq!(matching, Match::InProgress);
        }

               
    }*/


    mod test_function_check_word_with_regex{
        use super::*;
        #[test]
        fn test_simple_word() {
            let regex = Regex::new("hola").unwrap();
            assert_eq!(check_word_with_regex(&regex, "abcd"), false);
            assert_eq!(check_word_with_regex(&regex, "hola"), true);
            
        }

        mod test_wildcard_period{
            use super::*;

            #[test]
            fn test_wildcard_simple_period() {
                let regex = Regex::new(".").unwrap();
                assert_eq!(check_word_with_regex(&regex, "hola"), true);
                assert_eq!(check_word_with_regex(&regex, "hlla"), true);
                assert_eq!(check_word_with_regex(&regex, "abcd"), true);
            }        

            #[test]
            fn test_wildcard_period_1() {
                let regex = Regex::new(".a").unwrap();
                assert_eq!(check_word_with_regex(&regex, "aa"), true);
                assert_eq!(check_word_with_regex(&regex, "a"), false);
                assert_eq!(check_word_with_regex(&regex, "ab"), false);
                assert_eq!(check_word_with_regex(&regex, "ba"), true);
                assert_eq!(check_word_with_regex(&regex, "hola"), true);    
            }

            #[test]
            fn test_wildcard_period_2() {
                let regex = Regex::new("..a").unwrap();
                assert_eq!(check_word_with_regex(&regex, "hola"), true);
                assert_eq!(check_word_with_regex(&regex, "a"), false);
                assert_eq!(check_word_with_regex(&regex, "ab"), false);
                assert_eq!(check_word_with_regex(&regex, "ba"), false);
                assert_eq!(check_word_with_regex(&regex, "aa"), false);
                assert_eq!(check_word_with_regex(&regex, "aaa"), true);
            }

            #[test]
            fn test_wildcard_period_3() {
                let regex = Regex::new("b.").unwrap();
                assert_eq!(check_word_with_regex(&regex, "b"), false);
                assert_eq!(check_word_with_regex(&regex, "ab"), false);
                assert_eq!(check_word_with_regex(&regex, "ba"), true);
                assert_eq!(check_word_with_regex(&regex, "aa"), false);
                assert_eq!(check_word_with_regex(&regex, "bbb"), true);
            }

            #[test]
            fn test_wildcard_period_4() {
                let regex = Regex::new("b..").unwrap();
                assert_eq!(check_word_with_regex(&regex, "baa"), true);
                assert_eq!(check_word_with_regex(&regex, "b"), false);
                assert_eq!(check_word_with_regex(&regex, "ba"), false);
                assert_eq!(check_word_with_regex(&regex, "abab"), true);
                assert_eq!(check_word_with_regex(&regex, "bbb"), true);
            }
        }

        /*mod test_wildcard_repetition{
            use super::*;

            /*#[test]
            fn test_wildcard_rep_zero_or_one_not_valid(){
                //let regex = Regex::new("?");
                
            }*/

            #[test]
            fn test_wildcard_rep_zero_or_one(){
                let regex = Regex::new("a?").unwrap();
                assert_eq!(check_word_with_regex(&regex, "a"), true);
                assert_eq!(check_word_with_regex(&regex, "b"), true);
                assert_eq!(check_word_with_regex(&regex, "aa"), true);
                assert_eq!(check_word_with_regex(&regex, "ab"), true);
                assert_eq!(check_word_with_regex(&regex, "ba"), true);
                assert_eq!(check_word_with_regex(&regex, "bb"), true);
            }

            
            #[test]
            fn test_wildcard_repetition() {
                let regex = Regex::new("a*").unwrap();
                assert_eq!(check_word_with_regex(&regex, "a"), true);
                assert_eq!(check_word_with_regex(&regex, "aa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaaa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaaaa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaaaaa"), true);
            }

            #[test]
            fn test_wildcard_repetition_invalid() {
                let regex = Regex::new("*").unwrap();
                assert_eq!(check_word_with_regex(&regex, "b"), false);
                assert_eq!(check_word_with_regex(&regex, "ab"), false);
                assert_eq!(check_word_with_regex(&regex, "aaaaaab"), false);
            }

            #[test]
            fn test_wildcard_repetition_2() {
                let regex = Regex::new("a.*").unwrap();
                assert_eq!(check_word_with_regex(&regex, "a"), false);
                assert_eq!(check_word_with_regex(&regex, "aa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaaa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaaaa"), true);
                assert_eq!(check_word_with_regex(&regex, "aaaaaa"), true);
            }
        
            
        }*/
    }
}
