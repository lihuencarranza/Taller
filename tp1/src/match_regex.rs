/*use crate::regex::Regex;

struct MatchRegex {
    matched: String,
    expression: String,
}

fn match_regex(r: &Regex, s: String) -> bool {
    let mut matched = false;
    let mut match_regex = MatchRegex {
        matched: "".to_string(),
        expression: "".to_string(),
    };
    match r {
        Regex::Any => {
            matched = true;
            match_regex.matched = s;
            match_regex.expression = "Any".to_string();
        }
        Regex::Brackets(b) => {
            if b.contains(&s) {
                matched = true;
                match_regex.matched = s;
                match_regex.expression = format!("{:?}", b);
            }
        }
        Regex::ExactPlus(e) => {
            if e == &s {
                matched = true;
                match_regex.matched = s;
                match_regex.expression = format!("{:?}", e);
            }
        }
        Regex::Questionmark(q) => {
            if q == &s {
                matched = true;
                match_regex.matched = s;
                match_regex.expression = format!("{:?}", q);
            }
        }
        Regex::Range(r) => {
            if r.contains(&s) {
                matched = true;
                match_regex.matched = s;
                match_regex.expression = format!("{:?}", r);
            }
        }
        Regex::Wildcard => {
            matched = true;
            match_regex.matched = s;
            match_regex.expression = "Wildcard".to_string();
        }
    }
    if matched {
        println!("Matched: {:?} with expression: {:?}", match_regex.matched, match_regex.expression);
    }
    matched
}

pub fn compare_regex_with_expression(regexes: Vec<Regex>, s: String){
    for r in regexes.iter() {
        match_regex(r, s.to_string());
    }
}*/
