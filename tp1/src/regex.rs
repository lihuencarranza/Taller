use crate::any::handle_any;
use crate::brackets::handle_brackets;
use crate::exact_plus::handle_exact_plus;
use crate::questionmark::handle_zero_or_one;
use crate::range::handle_range;
use crate::special_char::handle_escape_sequence;
use crate::wildcard::handle_wildcard;

#[derive(Debug, PartialEq)]
pub enum RegexClass {
    Alpha,
    Alnum,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
}

#[derive(Debug, PartialEq)]
pub enum RegexValue {
    Literal(char),
    Wildcard, // comodin
    Class(RegexClass),
    Vowel,
    OneOf(Vec<char>),
}

#[derive(Debug, PartialEq)]
pub enum RegexRep {
    Any,
    Exact(usize),
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
    None,
}

#[derive(Debug, PartialEq)]
pub struct RegexStep {
    pub val: RegexValue,
    pub rep: RegexRep,
}

#[derive(Debug, PartialEq)]
pub struct Regex {
    steps: Vec<RegexStep>,
}

impl Regex {
    pub fn new(expression: &str) -> Result<Self, &str> {
        if !expression.is_ascii() {
            return Err("The expression is not ascii");
        }

        let mut steps: Vec<RegexStep> = vec![];
        let mut chars_iter = expression.chars();
        while let Some(c) = chars_iter.next() {
            let step = match c {
                '.' => handle_wildcard(),
                'a'..='z' | 'A'..='Z' | '0'..='9' => Some(RegexStep {
                    rep: RegexRep::Exact(1),
                    val: RegexValue::Literal(c),
                }),
                '?' => handle_zero_or_one(&mut steps)?,
                '*' => handle_any(&mut steps)?,
                '+' => handle_exact_plus(&mut steps)?,
                '{' => handle_range(&mut chars_iter, &mut steps)?,
                '[' => handle_brackets(&mut chars_iter)?,
                '^' => todo!(), // this means that the next character is not a special character
                '$' => todo!(), // this means that this is the end of the line
                '\\' => handle_escape_sequence(&mut chars_iter)?,
                _ => return Err("Invalid character"),
            };

            if let Some(p) = step {
                steps.push(p);
            }
        }

        Ok(Regex { steps })
    }
}

#[cfg(test)]
mod regex_tests {
    use super::*;

    mod regex_new {
        use super::*;

        #[test]
        fn literal() {
            let regex = Regex::new("abc").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('c'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn wildcard() {
            let regex = Regex::new("a*b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Any,
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn period() {
            let regex = Regex::new("a.b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Wildcard,
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn zero_or_one() {
            let regex = Regex::new("a?b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn one_or_more() {
            let regex = Regex::new("a+b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );

            let regex = Regex::new("ac+b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('c'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn n_times() {
            let regex = Regex::new("a{3}b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(3),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn from_n_times() {
            let regex = Regex::new("a{3,}b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Range {
                                min: Some(3),
                                max: None,
                            },
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn from_n_to_m_times() {
            let regex = Regex::new("a{3,5}b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Range {
                                min: Some(3),
                                max: Some(5),
                            },
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn to_m_times() {
            let regex = Regex::new("a{,5}b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Range {
                                min: None,
                                max: Some(5),
                            },
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn brackets_basic() {
            let regex = Regex::new("[abc]d").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::OneOf(vec!['a', 'b', 'c']),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('d'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn not_brackets() {
            let regex = Regex::new("[^abc]d").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::OneOf(vec!['a', 'b', 'c']),
                            rep: RegexRep::None,
                        },
                        RegexStep {
                            val: RegexValue::Literal('d'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );

            let regex = Regex::new("[^a-z]d").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Class(RegexClass::Lower),
                            rep: RegexRep::None,
                        },
                        RegexStep {
                            val: RegexValue::Literal('d'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn character_class() {
            let regex = Regex::new("[[:alpha:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Class(RegexClass::Alpha),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:alnum:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Class(RegexClass::Alnum),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:digit:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Class(RegexClass::Digit),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );
        }

        /*#[test]
        fn brackets_from_regex() {
            let regex = Regex::new("[abc]d").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::OneOf(vec!['a', 'b', 'c']),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('d'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn not_brackets() {
            let regex = Regex::new("[^abc]d").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Not(vec!['a', 'b', 'c']),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('d'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );

            let regex = Regex::new("[^a-z]d").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Not(vec![
                                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
                            ]),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('d'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }

        #[test]
        fn character_class() {
            let regex = Regex::new("[[:alpha:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Optional(vec![
                            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                            'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
                            'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
                        ]),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:alnum:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Optional(vec![
                            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                            'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
                            'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3',
                            '4', '5', '6', '7', '8', '9'
                        ]),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:digit:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Optional(vec![
                            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
                        ]),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:lower:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Optional(vec![
                            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                            'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
                        ]),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:upper:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Optional(vec![
                            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
                            'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
                        ]),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:space:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Optional(vec![' ', '\t', '\n', '\r', '\x0c', '\x0b']),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );

            let regex = Regex::new("[[:punct:]]").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![RegexStep {
                        val: RegexValue::Optional(vec![
                            '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.',
                            '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`',
                            '{', '|', '}', '~'
                        ]),
                        rep: RegexRep::Exact(1),
                    },]
                }
            );
        }

        #[test]
        fn random_tests() {
            let regex = Regex::new("a[[:alpha:]]b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Optional(vec![
                                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                                'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
                            ]),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );

            let regex = Regex::new("a[^[:alnum:]]b").unwrap();
            assert_eq!(
                regex,
                Regex {
                    steps: vec![
                        RegexStep {
                            val: RegexValue::Literal('a'),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Not(vec![
                                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                                'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
                            ]),
                            rep: RegexRep::Exact(1),
                        },
                        RegexStep {
                            val: RegexValue::Literal('b'),
                            rep: RegexRep::Exact(1),
                        },
                    ]
                }
            );
        }
        */
    }
}

// testear invalidos
