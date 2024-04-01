
#[derive(Debug, PartialEq)]
enum RegexClase{
    //
}


#[derive(Debug, PartialEq)]
enum RegexValue{
    Literal(char),
    Wildcard, // comodin
    //Clase(RegexClase),
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
struct RegexStep{
    val: RegexValue,
    rep: RegexRep,
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
                '.' => Some(RegexStep{
                    rep: RegexRep::Exact(1), 
                    val: RegexValue::Wildcard,
                }),
                'a'..='z' => Some(RegexStep{
                    rep: RegexRep::Exact(1),
                    val: RegexValue::Literal(c),
                }),
                '?' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Exact(1);
                    }else{
                        return Err("Se encontró un caracter '?' inesperado");
                    }
                    None
                }
                '*' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Any;
                    }else{
                        return Err("Se encontró un caracter '*' inesperado");
                    }
                   
                    None
                }
                '+' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Exact(1);
                    }else{
                        return Err("Se encontró un caracter '+' inesperado");
                    }
                    None
                }
                '{' => { // {n} Exact, {n,} From n, {n,m} Range, {,m} To m
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
                                let exact = parts[0].parse::<usize>().map_err(|_| "Failed to parse exact repetition")?;
                                last.rep = RegexRep::Exact(exact);
                            },
                            2 => {
                                let min = if parts[0].is_empty() {
                                    None
                                } else {
                                    Some(parts[0].parse::<usize>().map_err(|_| "Failed to parse min repetition")?)
                                };
                                let max = if parts[1].is_empty() {
                                    None
                                } else {
                                    Some(parts[1].parse::<usize>().map_err(|_| "Failed to parse max repetition")?)
                                };
                                last.rep = RegexRep::Range { min, max };
                            },
                            _ => return Err("Invalid repetition syntax"),
                        }
                    } else {
                        return Err("Unexpected '{' character");
                    }
                    None
                },
                '\\' => match chars_iter.next() {
                    Some(literal) => Some(
                        RegexStep{
                            rep: RegexRep::Exact(1),
                            val: RegexValue::Literal(literal),
                        }
                    ),
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
 
    }

        


}


// testear invalidos