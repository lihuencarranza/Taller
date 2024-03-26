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

