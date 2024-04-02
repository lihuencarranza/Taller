use crate::brackets::handle_brackets;
use crate::range::handle_range;
use crate::questionmark::handle_zero_or_one;
use crate::any::handle_any;
use crate::exact_plus::handle_exact_plus;


#[derive(Debug, PartialEq)]
pub enum RegexClass{
    Alpha,
    Alnum,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
}


#[derive(Debug, PartialEq)]
pub enum RegexValue{
    Literal(char),
    Wildcard, // comodin
    Class(RegexClass),
    Optional(Vec<char>),
}

#[derive(Debug, PartialEq)]
pub enum RegexRep{
    Any,
    Exact(usize), //{n}
    Range{
        min: Option<usize>,
        max: Option<usize>,
    },
    Negate,
}   

#[derive(Debug, PartialEq)]
pub struct RegexStep{
    pub val: RegexValue,
    pub rep: RegexRep,
}

#[derive(Debug, PartialEq)]
pub struct Regex {
    steps: Vec<RegexStep>
}

impl Regex {

    //slice : &str -> "hola"
    //string: String::from["hola"] 

    pub fn new(expression: &str) -> Result<Self, &str>{
        
        let mut steps: Vec<RegexStep> = vec![];
        let mut chars_iter = expression.chars();

        while let Some(c) = chars_iter.next(){
            
            let step = match c {
                '.' => 
                    Some(RegexStep{ rep: RegexRep::Exact(1), val: RegexValue::Wildcard,}),
                'a'..='z' => 
                    Some(RegexStep{  rep: RegexRep::Exact(1), val: RegexValue::Literal(c),}),
                '?' => 
                    handle_zero_or_one(&mut steps)?,
                '*' => 
                    handle_any(&mut steps)?,
                '+' => 
                    handle_exact_plus(&mut steps)?,
                '{' => 
                    handle_range(&mut chars_iter, &mut steps)?,
                '[' => 
                    handle_brackets(&mut chars_iter, &mut steps)?, 
                '\\' => match chars_iter.next() { Some(literal) => Some(
                        RegexStep{rep: RegexRep::Exact(1), val: RegexValue::Literal(literal),}),
                    None => return Err("se encontró un caracter inesperado") 
                },

                _ => return Err("Se encontró un caracter inesperado"),

            };

            if let Some(p) = step {
                steps.push(p);
            }
            
        }

        Ok(Regex { steps })
    }

    /*pub fn test(self, value: &str) -> Result<bool, &str>{
        
        if !value.is_ascii(){
            return Err("el input no es ASCII");
        }

        

        Ok(true)
    }*/

}

#[cfg(test)]
mod regex_tests {
    use super::*;

    
    mod regex_new{
        use super::*;

        #[test]
        fn literal(){
            let regex = Regex::new("abc").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('c'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        #[test]
        fn wildcard(){
            let regex = Regex::new("a*b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Any,
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        #[test]
        fn period(){
            let regex = Regex::new("a.b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Wildcard,
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        #[test]
        fn zero_or_one(){
            let regex = Regex::new("a?b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }
        
        #[test]
        fn one_or_more(){
            let regex = Regex::new("a+b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });

            let regex = Regex::new("ac+b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('c'),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }
 
        #[test]
        fn n_times(){
            let regex = Regex::new("a{3}b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Exact(3),
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        #[test]
        fn from_n_times(){
            let regex = Regex::new("a{3,}b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Range{
                            min: Some(3),
                            max: None,
                        },
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        #[test]
        fn from_n_to_m_times(){
            let regex = Regex::new("a{3,5}b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Range{
                            min: Some(3),
                            max: Some(5),
                        },
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        #[test]
        fn to_m_times(){
            let regex = Regex::new("a{,5}b").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Literal('a'),
                        rep: RegexRep::Range{
                            min: None,
                            max: Some(5),
                        },
                    },
                    RegexStep{
                        val: RegexValue::Literal('b'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }
 

        #[test]
        fn brackets(){
            let regex = Regex::new("[abc]d").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Optional(vec!['a', 'b', 'c']),
                        rep: RegexRep::Exact(1),
                    },
                    RegexStep{
                        val: RegexValue::Literal('d'),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        
        /*#[test]
        fn character_class(){
            let regex = Regex::new("[[:alpha:]]").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Clase(RegexClase::Alpha),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });

            let regex = Regex::new("[[:alnum:]]").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Clase(RegexClase::Alnum),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });

            let regex = Regex::new("[[:digit:]]").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Clase(RegexClase::Digit),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });

            let regex = Regex::new("[[:lower:]]").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Clase(RegexClase::Lower),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });

            let regex = Regex::new("[[:upper:]]").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Clase(RegexClase::Upper),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });

            let regex = Regex::new("[[:space:]]").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Clase(RegexClase::Space),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });

            let regex = Regex::new("[[:punct:]]").unwrap();
            assert_eq!(regex, Regex{
                steps: vec![
                    RegexStep{
                        val: RegexValue::Clase(RegexClase::Punct),
                        rep: RegexRep::Exact(1),
                    },
                ]
            });
        }

        */


        
    }

        


}


// testear invalidos