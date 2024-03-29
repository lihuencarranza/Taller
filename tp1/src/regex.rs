#[derive(Debug, PartialEq)]
pub enum MetacharClass {
    Period,
    None
}

#[derive(Debug, PartialEq)]
pub enum State {
    InProgress,
    Completed,
    NotStarted,
}


pub struct Regex {
    literal: Vec<char>,
    metachar: Vec<MetacharClass>,
}

impl Regex {
    pub fn new(expression: &str) -> Result<Self, &str> {
        if expression.is_empty() {
            return Err("Empty expression");
        }

        let literal: Vec<char> = expression.chars().collect();
        let mut metachar: Vec<MetacharClass> = vec![];

        let mut chars_iter = expression.chars();
        while let Some(c) = chars_iter.next() {
            let metachar_class = match c {
                '.' => MetacharClass::Period,
                _ => MetacharClass::None,
            };
            metachar.push(metachar_class);
        }


        Ok(Regex {
            literal,
            metachar,
        })
    }
}

pub fn check_regex_in_list(regex: Regex, list: &Vec<String>) -> Vec<String>{
    let mut list_coincidences: Vec<String> = Vec::new();
    for word in list{
        let mut word_iter = word.chars();
        let mut regex_literal_iter = regex.literal.iter();
        let mut metachar_iter = regex.metachar.iter();

        if check_word_with_regex(&mut word_iter, &mut regex_literal_iter, &mut metachar_iter){
            list_coincidences.push(word.to_string());
        }
    }
    list_coincidences
}

fn check_word_with_regex(word_iter: &mut std::str::Chars, mut regex_literal_iter: &mut std::slice::Iter<char>, metachar_iter: &mut std::slice::Iter<MetacharClass>) -> bool{

    let regex_size = check_lenght_word_with_regex(word_iter, regex_literal_iter);
    if regex_size == -1 {
        return false;
    } 

    
    let regex_literal_iter_initial = regex_literal_iter.clone();
    //variable si empezó a matchear
    let mut match_started: State = State::NotStarted;

    while (match_started != State::Completed){
        
        let mut char_word = word_iter.next();
        let mut char_regex = regex_literal_iter.next();
        
        //si la palabra no tiene mas caracteres pero la expresion regular si
        if char_word.is_none() && char_regex.is_some(){
            return false;
        }
        //si la expresion regular no tiene más caracteres devuelve true
        if char_regex.is_none() && match_started == State::InProgress{
            match_started = State::Completed;
            break;
        }else if char_regex.is_none() && match_started != State::InProgress { 
            break;
        }
        // si el char de la palabra coincide con el de la regex, avanza el iterador de la palabra y la regex
        if char_word == char_regex.copied(){
            match_started = State::InProgress;
            continue;
        }else{
            match_started = State::NotStarted;
            *regex_literal_iter = regex_literal_iter_initial.clone();
        }
    } 
    
    return match_started == State::Completed;

    /*{
        
        //si el caracter de la regex y no coincide con el de la palabra word devuelve false
        
        //si el caracter de la regex es un '.' y el de la palabra no es un '\n' sigue al siguiente char de la palabra
        //ademas elimina el metachar de la lista de metachars
        /*if let Some(MetacharClass::Period) = metachar_iter.next(){
            if c == '\n'{
                return false;
            }
            continue;
        }*/
        //si el caracter de la regex y no coincide con el de la palabra word devuelve false
        
        
        
    }*/
}

fn check_lenght_word_with_regex(word_iter: &mut std::str::Chars, regex_literal_iter: &mut std::slice::Iter<char>)-> i32{
    let regex_size = regex_literal_iter.clone().count();
    //si la expresion es mas larga que la palabra devuelve false
    let word_size = word_iter.clone().count();
    if regex_size > word_size{
        return -1;
    }
    regex_size as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_correct_new() {
        let expression = "a";
        let regex = Regex::new(expression).unwrap();
        assert_eq!(regex.literal, vec!['a']);
        assert_eq!(regex.metachar, vec![MetacharClass::None]);
        //assert_eq!(regex.state, State::NotStarted);
    }
}

#[test]
fn test_regex_incorrect_new() {
    let regex = Regex::new("");
    match regex {
        Ok(_) => {
            panic!("La expresión regular no debería ser válida");
        }
        Err(e) => {
            assert_eq!(e, "Empty expression");
        }
    }
}

#[test]
fn test_regex_correct_check_regex_in_list() {
    let expression = "a";
    let regex = Regex::new(expression).unwrap();
    let list = vec!["a".to_string(), "b".to_string()];
    let list_coincidences = check_regex_in_list(regex, &list);
    assert_eq!(list_coincidences, vec!["a".to_string()]);
}

#[test]
fn test_regex_incorrect_check_regex_in_list() {
    let expression = "a";
    let regex = Regex::new(expression).unwrap();
    let list = vec!["b".to_string()];
    //let list_coincidences: Vec<String> = check_regex_in_list(regex, &list);

    //assert_eq!(list_coincidences, vec![]);
}

