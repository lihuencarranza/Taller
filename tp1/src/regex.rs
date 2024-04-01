use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum RegexClase{
    //
}

#[derive(Debug, PartialEq)]
enum RepType{
    Period,
    ZeroOrMore,
    ZeroOrOne,
}

#[derive(Debug, PartialEq)]
enum RegexValue{
    Literal(char),
    Wildcard, // comodin
    Clase(RegexClase),
    Repetition(RepType),
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

    pub fn test(self, value: &str) -> Result<bool, &str>{
        
        if !value.is_ascii(){
            return Err("el input no es ASCII");
        }

        

        Ok(true)
    }

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


        
        
    }

        


}


// testear invalidos