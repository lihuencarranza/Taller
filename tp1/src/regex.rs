struct RegexStep{
        val: RegexVal,
        rep: RegexRep,
}

pub struct Regex{
        steps: Vec<RegexStep>,
}

enum RegexVal{
        Literal(char),
        Any,
}

enum RegexRep{
        Any,
        Exact(unize),
        Range{
                min: Option<usize>,
                max: Option<usize>,
        }
}

impl Regex {
        pub fn new(expression: &str) -> Self{
                let mut steps: Vec<RegexStep> = vec![];

                let mut chars_iter : Chars<'_> = expression.chars();
                chars_iter.next();
                while let Some(c) = chars_iter.next(){
                        let val = match c{
                                '.' => RegexVal::Any,
                                _ => RegexVal::Literal(c),
                        };
                        let rep = match chars_iter.next(){
                                Some('*') => RegexRep::Any,
                                Some('+') => RegexRep::Range{min: Some(1), max: None},
                                Some('?') => RegexRep::Range{min: Some(0), max: Some(1)},
                                Some('{') => {
                                        let mut min = None;
                                        let mut max = None;
                                        let mut num = String::new();
                                        while let Some(c) = chars_iter.next(){
                                                if c == '}'{
                                                        break;
                                                }
                                                num.push(c);
                                        }
                                        let mut nums = num.split(',');
                                        min = nums.next().map(|x| x.parse().unwrap());
                                        max = nums.next().map(|x| x.parse().unwrap());
                                        RegexRep::Range{min, max}
                                },
                                _ => RegexRep::Exact(1),
                        };
                        steps.push(RegexStep{val, rep});
                }

        }
}

