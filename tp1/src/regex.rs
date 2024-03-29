use std::io::empty;

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
    let mut previous_step: char = ' ';
    let mut is_metachar = false;

    while match_started != State::Completed {
        
        let char_word = word_iter.next();
        let char_regex = regex_literal_iter.next();
        is_metachar = false;
        
        //si la palabra no tiene mas caracteres pero la expresion regular si
        if char_word.is_none() && char_regex.is_some(){
            return false;
        } else if char_word.is_none() && char_regex.is_none(){
            if match_started == State::InProgress{
                match_started = State::Completed;
            } 
            break;
        }
        //si la expresion regular no tiene más caracteres devuelve true
        if char_regex.is_none() && match_started == State::InProgress{
            match_started = State::Completed;
            break;
        }else if char_regex.is_none() && match_started != State::InProgress { 
            break;
        }

        save_char_in_previous_step(&mut previous_step, &char_word);

        //si el char de la regex es un '.' avanza el iterador de la palabra y la regex
        // si es ',' termina el match
        match char_regex {
            Some('.') => {
                match_started = State::InProgress;
                metachar_iter.next();
                is_metachar = true;
                if regex_literal_iter.clone().count() == 0{
                    match_started = State::Completed;  
                    break;
                }
            }
            _ => {
                if char_word == char_regex.copied(){
                    match_started = State::InProgress;
                } else {
                    match_started = State::NotStarted;
                    *regex_literal_iter = regex_literal_iter_initial.clone();
                }
            },
        };

        // si el char de la palabra coincide con el de la regex, avanza el iterador de la palabra y la regex
       
        /*if regex_literal_iter.clone().count() == 0{

            if word_iter.clone().count() == 0 && match_started == State::InProgress{
                match_started = State::Completed;  
            }
            //break;
        }*/


        
    } 
    
    return match_started == State::Completed;

    
}

fn save_char_in_previous_step(previous_step: &mut char, char_word: &Option<char>){
    if let Some(c) = char_word {
        *previous_step = *c;
    }
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
        let list_coincidences: Vec<String> = check_regex_in_list(regex, &list);
        //assert_eq!(list_coincidences, vec![]);
    }

    #[test]
    fn test_regex_correct_check_word_with_regex() {
        let expression = "a";
        let regex = Regex::new(expression).unwrap();
        let word = "a";
        let mut word_iter = word.chars();
        let mut regex_literal_iter = regex.literal.iter();
        let mut metachar_iter = regex.metachar.iter();
        let result = check_word_with_regex(&mut word_iter, &mut regex_literal_iter, &mut metachar_iter);
        assert_eq!(result, true);
    }

    #[test]
    fn test_regex_incorrect_check_word_with_regex() {
        let expression = "a";
        let regex = Regex::new(expression).unwrap();
        let word = "b";
        let mut word_iter = word.chars();
        let mut regex_literal_iter = regex.literal.iter();
        let mut metachar_iter = regex.metachar.iter();
        let result = check_word_with_regex(&mut word_iter, &mut regex_literal_iter, &mut metachar_iter);
        assert_eq!(result, false);
    }

    //list of coincidences
    #[test]
    fn test_regex_correct_check_word_with_regex_list() {
        let expression = "a";
        let regex = Regex::new(expression).unwrap();
        let list = vec!["a".to_string(), "b".to_string()];
        let list_coincidences = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec!["a".to_string()]);
    }

    #[test]
    fn test_regex_incorrect_check_word_with_regex_list() {
        let expression = "a";
        let regex = Regex::new(expression).unwrap();
        let list = vec!["b".to_string()];
        let list_coincidences: Vec<String> = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec![] as Vec<String>);
    }

    #[test]
    fn test_regex_char_expressions() {
        let expression = "ab";
        let regex = Regex::new(expression).unwrap();
        let list: Vec<String> = vec!["abbcd".to_string(), "ab".to_string(), "abcd ".to_string(), "ac".to_string(), "bc".to_string(), "dd".to_string(), "cdab".to_string(), "abgcd".to_string(), "abggcd".to_string()];
        let list_coincidences = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec!["abbcd".to_string(), "ab".to_string(), "abcd ".to_string(), "cdab".to_string(), "abgcd".to_string(), "abggcd".to_string()]);
    }
    
    #[test]
    fn test_regex_metachar_simple_point_expressions() {
        let expression = "a.";
        let regex = Regex::new(expression).unwrap();
        let list: Vec<String> = vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()];
        let list_coincidences = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()]);
    }

    #[test]
    fn test_regex_metachar_simple_point_expressions_2() {
        let expression = "ab.";
        let regex = Regex::new(expression).unwrap();
        let list: Vec<String> = vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()];
        let list_coincidences = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec!["abbcd".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()]);
    }

    #[test]
    fn test_regex_metachar_simple_point_expressions_3() {
        let expression = "a...d";
        let regex = Regex::new(expression).unwrap();
        let list: Vec<String> = vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()];
        let list_coincidences = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec!["abbcd".to_string(), "abgcd".to_string()]);
    }

    #[test]
    fn test_regex_metachar_simple_point_expressions_4() {
        let expression = "a..d";
        let regex = Regex::new(expression).unwrap();
        let list: Vec<String> = vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string(), "abcd".to_string()];
        let list_coincidences = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec!["abcd".to_string()]);
    }

    #[test]
    fn test_regex_metachar_simple_point_expressions_5() {
        let expression = ".b";
        let regex = Regex::new(expression).unwrap();
        let list: Vec<String> = vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()];
        let list_coincidences = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()]);
    }

    #[test]
    fn test_regex_metachar_simple_point_expressions_6() {
        let expression = "a.bb";
        let regex = Regex::new(expression).unwrap();
        let list: Vec<String> = vec!["abbcd".to_string(), "ab".to_string(), "abc".to_string(), "abgcd".to_string(), "abggcd".to_string()];
        let list_coincidences: Vec<String> = check_regex_in_list(regex, &list);
        assert_eq!(list_coincidences, vec![] as Vec<String>);
    }


}






