
#[derive(Debug, PartialEq)]
enum RegexClass{
    Period,
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
    }
}

#[derive(Debug, PartialEq)]
pub struct RegexStep{
    val: RegexValue,
    rep: RegexRep,
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

        while let Some(c) = chars_iter.next(){
            
            let step = match c {
                '.' => Some(RegexStep{
                    rep: RegexRep::Exact(1), 
                    val: RegexValue::Wildcard(RegexClass::Period),
                }),
                'a'..='z' => Some(RegexStep{
                    rep: RegexRep::Exact(1),
                    val: RegexValue::Literal(c),
                }),
                
                _ => return Err("Invalid character in expression"),

            };

            if let Some(p) = step {
                steps.push(p);
            }
        }

		Ok(Regex{
            steps
        })
	}
}

pub fn check_word_with_regex(regex: &Regex, word: &str) -> bool {
    let mut word_iter = word.chars();
    let mut steps_iter = regex.steps.iter();

    while let Some(c) = word_iter.next(){
        if let Some(step) = steps_iter.next(){
            
            if let RegexValue::Literal(l) = step.val {
                if l != c {
                    return false;
                }
            } else {
                if check_wilcard_type(step, &mut word_iter, c){
                    continue;
                }
            }
                
            
        }
    }

    true
}

pub fn check_wilcard_type(step: &RegexStep, word_iter: &mut std::str::Chars, c: char) -> bool{
    match step.rep {
        RegexRep::Exact(1) => {
            match step.val {
                RegexValue::Wildcard(RegexClass::Period) => {
                    return true;
                },
                _ => return false,
            }
        },
        _ => return false,
    }
}



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
            };
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_eq!(step.val, RegexValue::Literal('a'));
        }

        
    }

    mod regex_value_tests{
        use super::*;
        #[test]
        fn test_regex_value_literal_from_regex() {
            let regex = Regex::new("hola").unwrap();
            let step = &regex.steps[0];
            assert_eq!(step.val, RegexValue::Literal('h'));
            assert_eq!(step.rep, RegexRep::Exact(1));
            assert_ne!(step.val, RegexValue::Wildcard(RegexClass::Period));
        }

        #[test]
        fn test_regex_value_wildcard_from_regex() {
            let regex = Regex::new(".").unwrap();
            let step = &regex.steps[0];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
        }    

        #[test]
        fn test_regex_value_literal_from_regex_with_wildcard() {
            let regex = Regex::new("..a").unwrap();
            let step = &regex.steps[2];
            assert_eq!(step.val, RegexValue::Literal('a'));
            assert_eq!(step.rep, RegexRep::Exact(1));
            let step = &regex.steps[1];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
        }

        #[test]
        fn test_regex_value_literal_from_regex_with_wildcard_and_literal() {
            let regex = Regex::new("b.").unwrap();
            let step = &regex.steps[1];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
        }

        #[test]
        fn test_regex_value_literal_from_regex_with_wildcard_and_literal_and_wildcard() {
            let regex = Regex::new("b..").unwrap();
            let step = &regex.steps[2];
            assert_eq!(step.val, RegexValue::Wildcard(RegexClass::Period));
            assert_eq!(step.rep, RegexRep::Exact(1));
        }

    }


    mod test_check_word_with_regex{
        use super::*;
        #[test]
        fn test_check_simple_word_with_regex() {
            let regex = Regex::new("hola").unwrap();
            assert_eq!(check_word_with_regex(&regex, "hola"), true);
            assert_ne!(check_word_with_regex(&regex, "abcd"), true);
        }

        #[test]
        fn test_check_word_with_regex_with_wildcard() {
            let regex = Regex::new(".").unwrap();
            assert_eq!(check_word_with_regex(&regex, "hola"), true);
            assert_eq!(check_word_with_regex(&regex, "hlla"), true);
            assert_eq!(check_word_with_regex(&regex, "abcd"), true);
        }

               
    }
}
