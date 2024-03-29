#[derive(Debug)]
pub enum MetacharClass {
    Period,
    None
}

#[derive(Debug)]
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

fn check_word_with_regex(word_iter: &mut std::str::Chars, regex_literal_iter: &mut std::slice::Iter<char>, metachar_iter: &mut std::slice::Iter<MetacharClass>) -> bool{

    let regex_size = regex_literal_iter.clone().count();

    for mut i in 0..regex_size{
        
        let char_word = word_iter.next();
        let char_regex = regex_literal_iter.next();
        
        //si la palabra no tiene mas caracteres pero la expresion regular si
        if char_word.is_none() && char_regex.is_some(){
            return false;
        }
        //si la expresion regular no tiene más caracteres devuelve true
        if char_regex.is_none(){
            return true;
        }
        // si el char de la palabra coincide con el de la regex, continua
        if char_word != char_regex.copied(){
            return false;
        }

    }

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
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_correct_new() {
        let expression = "a";
        let regex = Regex::new(expression).unwrap();
        assert_eq!(regex.literal, vec!['a']);
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
}
