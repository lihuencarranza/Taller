use crate::metachars::RegexClass;
use crate::regex::Regex;
use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use crate::regex_val::RegexValue;
use crate::type_of_line::RegexRestriction;

/// Enum to represent the state of the match
#[derive(Debug, PartialEq)]
pub enum MatchState {
    /// The match is not completed
    NotMatched,
    /// The match is in progress
    InProgress,
    /// The match is at the end of the regex
    EndOfRegex,
    /// The match is at the end of the word
    EndOfWord,
}

/// Checks the regex class with the actual char of the word
/// - If the class is Alpha and the char is "a", returns true
/// - If the class is Upper and the char is "a", returns false
/// # Arguments
/// * `class` - A reference to a RegexClass
/// * `c` - A char
/// # Returns
/// * A boolean
/// # Example
/// let class = RegexClass::Alpha;
/// let c = 'a';
/// let result = handle_regex_class(&class, c);
/// assert_eq!(result, true);
pub fn handle_regex_class(class: &RegexClass, c: char) -> bool {
    match class {
        RegexClass::Alpha => c.is_ascii_alphabetic(),
        RegexClass::Alnum => c.is_ascii_alphanumeric(),
        RegexClass::Digit => c.is_ascii_digit(),
        RegexClass::Lower => c.is_ascii_lowercase(),
        RegexClass::Upper => c.is_ascii_uppercase(),
        RegexClass::Punct => c.is_ascii_punctuation(),
        RegexClass::Space => c.is_ascii_whitespace(),
    }
}

/// Checks if the backtracking is the end of the line ( $ )
/// # Arguments
/// * `backtracking` - A reference to an `Option<Vec<RegexRestriction>>`
/// # Returns
/// * A boolean
/// # Example
/// let backtracking = Some(vec![RegexRestriction::EndOfLine]);
/// let result = is_end_of_line(&backtracking);
/// assert_eq!(result, true);
pub fn is_end_of_line(backtracking: &Option<Vec<RegexRestriction>>) -> bool {
    if let Some(restrictions) = backtracking {
        for restriction in restrictions {
            match restriction {
                RegexRestriction::EndOfLine => return true,
                _ => continue,
            }
        }
    }

    false
}

/// Checks if the backtracking is the start of the line ( ^ )
/// # Arguments
/// * `backtracking` - A reference to an `Option<Vec<RegexRestriction>>`
/// # Returns
/// * A boolean
/// # Example
/// let backtracking = Some(vec![RegexRestriction::StartOfLine]);
/// let result = is_start_of_line(&backtracking);
/// assert_eq!(result, true);
pub fn is_start_of_line(backtracking: &Option<Vec<RegexRestriction>>) -> bool {
    if let Some(restrictions) = backtracking {
        for restriction in restrictions {
            match restriction {
                RegexRestriction::StartOfLine => return true,
                _ => continue,
            }
        }
    }

    false
}

/// Handles the case when the match is at the end of the expression
/// # Arguments
/// * `match_state` - A MatchState
/// * `regex` - A reference to a Regex
/// * `word` - A reference to a str
/// * `input_chars` - A mutable reference to a std::str::Chars
/// * `result` - A mutable reference to a String
/// # Returns
/// * A String
/// # Example
/// let match_state = MatchState::EndOfRegex;
/// let regex = Regex::new("a").unwrap();
/// let word = "a";
/// let mut input_chars = word.chars();
/// let result = "".to_string();
/// let result = handle_end_of_expression(match_state, &regex, word, &mut input_chars, result);
/// assert_eq!(result, "a".to_string());
pub fn handle_end_of_expression(
    match_state: MatchState,
    regex: &Regex,
    word: &str,
    input_chars: &mut std::str::Chars,
    result: String,
) -> String {
    if match_state == MatchState::EndOfRegex {
        if is_end_of_line(&regex.backtracking) && input_chars.next().is_some() {
            return compare_regex_with_expression(regex, &word[1..]);
        }
        return result;
    }

    if !is_start_of_line(&regex.backtracking) && input_chars.next().is_some() {
        return compare_regex_with_expression(regex, &word[1..]);
    }

    "".to_string()
}

/// Handles the case when the step is none, this means that the character received should be in this step
/// # Arguments
/// * `step` - A reference to a RegexStep
/// * `actual_char` - A char
/// * `match_state` - A mutable reference to a MatchState
/// # Returns
/// * A boolean
/// # Example
/// let step = RegexStep { rep: RegexRep::None, val: RegexValue::Literal('a') };
/// let actual_char = 'a';
/// let mut match_state = MatchState::InProgress;
/// let result = handle_none_case(&step, actual_char, &mut match_state);
/// assert_eq!(result, false);
/// let step = RegexStep { rep: RegexRep::None, val: RegexValue::Literal('a') };
/// let actual_char = 'b';
/// let mut match_state = MatchState::InProgress;
/// let result = handle_none_case(&step, actual_char, &mut match_state);
/// assert_eq!(result, true);
pub fn handle_none_case(step: &RegexStep, actual_char: char, match_state: &mut MatchState) -> bool {
    match &step.val {
        RegexValue::Literal(c) => {
            if c == &actual_char {
                *match_state = MatchState::NotMatched;
                return false;
            }
        }
        RegexValue::Wildcard => {
            *match_state = MatchState::NotMatched;
            return false;
        }
        RegexValue::Class(class) => {
            if handle_regex_class(class, actual_char) {
                *match_state = MatchState::NotMatched;
                return false;
            }
        }
        RegexValue::OneOf(chars) => {
            if chars.contains(&actual_char) {
                *match_state = MatchState::NotMatched;
                return false;
            }
        }
    }
    *match_state = MatchState::InProgress;
    true
}

/// Handles the case when the step is exact, this means that the character received should be in this step
/// # Arguments
/// * `step` - A reference to a RegexStep
/// * `actual_char` - A char
/// * `match_state` - A mutable reference to a MatchState
/// * `count` - A usize
/// * `input_chars` - A mutable reference to a std::str::Chars
/// * `result` - A mutable reference to a String
/// # Returns
/// * A boolean
/// # Example
/// let step = RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') };
/// let actual_char = 'a';
/// let mut match_state = MatchState::InProgress;
/// let count = 1;
/// let mut input_chars = "a".chars();
/// let result = "".to_string();
/// let result = handle_exact_case(&step, actual_char, &mut match_state, count, &mut input_chars, result);
/// assert_eq!(result, true);
pub fn handle_exact_case(
    step: &RegexStep,
    mut actual_char: char,
    match_state: &mut MatchState,
    count: usize,
    input_chars: &mut std::str::Chars,
    result: &mut String,
) -> bool {
    for index in 0..count {
        match &step.val {
            RegexValue::Literal(c) => {
                if c != &actual_char {
                    *match_state = MatchState::NotMatched;
                    break;
                }
            }
            RegexValue::Wildcard => {}
            RegexValue::Class(class) => {
                if !handle_regex_class(class, actual_char) {
                    *match_state = MatchState::NotMatched;
                    break;
                }
            }
            RegexValue::OneOf(content) => {
                if !content.contains(&actual_char) {
                    *match_state = MatchState::NotMatched;
                    break;
                }
            }
        }

        *match_state = MatchState::InProgress;
        result.push(actual_char);
        if index < count - 1 {
            actual_char = match input_chars.next() {
                Some(c) => c,
                None => {
                    *match_state = MatchState::EndOfWord;
                    break;
                }
            };
        }
    }
    if *match_state == MatchState::NotMatched {
        return false;
    }
    true
}

/// Sets and matches the minimum number of characters that should be matched
/// - If the minimum number of characters is not matched, it returns None
pub fn handle_min(
    step: &RegexStep,
    actual_char: char,
    match_state: &mut MatchState,
    input_chars: &mut std::str::Chars,
    result: &mut String,
    min: Option<usize>,
) -> Option<usize> {
    match min {
        Some(min) => {
            if !handle_exact_case(step, actual_char, match_state, min, input_chars, result) {
                None
            } else {
                Some(min)
            }
        }
        None => Some(0),
    }
}

/// Sets the maximum number of characters that can be matched
/// - If there was specified a maximum number of characters, it returns the difference between the maximum and the minimum
/// - If there wasn't specified a maximum number of characters, it returns the maximum value of i8
pub fn handle_max(min_count: usize, max: Option<usize>) -> i8 {
    match max {
        Some(max_count) => max_count as i8 - min_count as i8,
        None => i8::MAX,
    }
}

/// Handles the case when the step is none
/// - The character that's being analize should be saved in the result if it's found
/// # Arguments
/// * `step` - A reference to a RegexStep
/// * `actual_char` - A char
/// * `match_state` - A mutable reference to a MatchState
/// * `count` - A usize
/// * `input_chars` - A mutable reference to a std::str::Chars
/// * `result` - A mutable reference to a String
/// # Returns
/// * A boolean
/// # Example
/// let step = RegexStep { rep: RegexRep::None, val: RegexValue::Literal('a') };
/// let actual_char = 'a';
/// let mut match_state = MatchState::InProgress;
/// let count = 1;
/// let mut input_chars = "a".chars();
/// let result = "".to_string();
/// let result = handle_none_case(&step, actual_char, &mut match_state);
/// assert_eq!(result, false);
/// let step = RegexStep { rep: RegexRep::None, val: RegexValue::Literal('a') };
/// let actual_char = 'b';
/// let mut match_state = MatchState::InProgress;
/// let count = 1;
/// let mut input_chars = "b".chars();
/// let result = "".to_string();
/// let result = handle_none_case(&step, actual_char, &mut match_state);
/// assert_eq!(result, true);
pub fn handle_main_loop(
    step: &RegexStep,
    mut new_char: char,
    max_count: i8,
    count: &mut i8,
    input_chars: &mut std::str::Chars,
    result: &mut String,
    match_state: &mut MatchState,
) -> bool {
    let mut new_result = String::new();
    while *count <= max_count {
        if handle_exact_case(step, new_char, match_state, 1, input_chars, &mut new_result) {
            result.push_str(&new_result);
            return true;
        }
        result.push(new_char);
        new_char = match input_chars.next() {
            Some(c) => c,
            None => {
                *match_state = MatchState::EndOfWord;
                return true;
            }
        };

        *count += 1;
    }
    true
}

/// Handles the case when the step is a range
/// - The character that's being analize should be saved in the result if it's found
/// - If the range has multiple characters that match, all should be saved in the result
/// # Arguments
/// * `step` - A reference to a RegexStep
/// * `steps_iter` - A mutable reference to a `std::slice::Iter<RegexStep>`
/// * `actual_char` - A char
/// * `match_state` - A mutable reference to a MatchState
/// * `input_chars` - A mutable reference to a std::str::Chars
/// * `result` - A mutable reference to a String
/// * `min` - An Option of a usize
/// * `max` - An Option of a usize
/// # Returns
/// * A boolean
/// # Example
/// let step = RegexStep { rep: RegexRep::Range { min: Some(1), max: Some(2) }, val: RegexValue::Literal('a') };
/// let mut steps_iter = vec![step].iter();
/// let actual_char = 'a';
/// let mut match_state = MatchState::InProgress;
/// let mut input_chars = "a".chars();
/// let result = "".to_string();
/// let min = Some(1);
/// let max = Some(2);
/// let result = handle_range_case(&step, &mut steps_iter, actual_char, &mut match_state, &mut input_chars, result, min, max);
/// assert_eq!(result, true);
/// let step = RegexStep { rep: RegexRep::Range { min: Some(1), max: Some(2) }, val: RegexValue::Literal('a') };
/// let mut steps_iter = vec![step].iter();
/// let actual_char = 'b';
/// let mut match_state = MatchState::InProgress;
/// let mut input_chars = "b".chars();
/// let result = "".to_string();
/// let min = Some(1);
/// let max = Some(2);
/// let result = handle_range_case(&step, &mut steps_iter, actual_char, &mut match_state, &mut input_chars, result, min, max);
/// assert_eq!(result, false);
#[allow(clippy::too_many_arguments)]
pub fn handle_range_case(
    step: &RegexStep,
    steps_iter: &mut std::slice::Iter<RegexStep>,
    actual_char: char,
    match_state: &mut MatchState,
    input_chars: &mut std::str::Chars,
    result: &mut String,
    min: Option<usize>,
    max: Option<usize>,
) -> bool {
    let min_count = match handle_min(step, actual_char, match_state, input_chars, result, min) {
        Some(min_count) => min_count,
        None => return false,
    };

    let max_count = handle_max(min_count, max);
    let mut count = 0;

    let step = match steps_iter.next() {
        Some(s) => s,
        None => {
            *match_state = MatchState::EndOfRegex;
            for c in input_chars.by_ref() {
                if Some(c) == Some(actual_char) && count < max_count {
                    result.push(c);
                    count += 1;
                } else {
                    break;
                }
            }
            return true;
        }
    };

    let new_char = match input_chars.next() {
        Some(c) => c,
        None => {
            *match_state = MatchState::EndOfWord;
            return true;
        }
    };

    handle_main_loop(
        step,
        new_char,
        max_count,
        &mut count,
        input_chars,
        result,
        match_state,
    )
}

/// Handles the case the step's repetition
/// - The character that's being analize should be saved in the result if it's found
/// # Arguments
/// * `step` - A reference to a RegexStep
/// * `steps_iter` - A mutable reference to a `std::slice::Iter<RegexStep>`
/// * `actual_char` - A char
/// * `match_state` - A mutable reference to a MatchState
/// * `input_chars` - A mutable reference to a std::str::Chars
/// * `result` - A mutable reference to a String
/// # Returns
/// * A boolean
/// # Example
/// let step = RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') };
/// let mut steps_iter = vec![step].iter();
/// let actual_char = 'a';
/// let mut match_state = MatchState::InProgress;
/// let mut input_chars = "a".chars();
/// let result = "".to_string();
/// let result = handle_step_rep(&step, &mut steps_iter, actual_char, &mut match_state, &mut input_chars, result);
/// assert_eq!(result, true);
/// let step = RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') };
/// let mut steps_iter = vec![step].iter();
/// let actual_char = 'b';
/// let mut match_state = MatchState::InProgress;
/// let mut input_chars = "b".chars();
/// let result = "".to_string();
/// let result = handle_step_rep(&step, &mut steps_iter, actual_char, &mut match_state, &mut input_chars, result);
/// assert_eq!(result, false);
pub fn handle_step_rep(
    step: &RegexStep,
    steps_iter: &mut std::slice::Iter<RegexStep>,
    actual_char: char,
    match_state: &mut MatchState,
    input_chars: &mut std::str::Chars,
    result: &mut String,
) -> bool {
    match step.rep {
        RegexRep::Exact(count) => {
            handle_exact_case(step, actual_char, match_state, count, input_chars, result)
        }
        RegexRep::Range { min, max } => handle_range_case(
            step,
            steps_iter,
            actual_char,
            match_state,
            input_chars,
            result,
            min,
            max,
        ),

        RegexRep::None => {
            if handle_none_case(step, actual_char, match_state) {
                result.push(actual_char);
            }
            true
        }
    }
}

///  Processes the regex steps
/// # Arguments
/// * `steps_iter` - A mutable reference to a `std::slice::Iter<RegexStep>`
/// * `input_chars` - A mutable reference to a std::str::Chars
/// * `match_state` - A mutable reference to a MatchState
/// * `result` - A mutable reference to a String
/// # Example
/// let mut steps_iter = vec![RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') }].iter();
/// let mut input_chars = "a".chars();
/// let mut match_state = MatchState::InProgress;
/// let result = "".to_string();
/// process_regex_steps(&mut steps_iter, &mut input_chars, &mut match_state, &mut result);
/// assert_eq!(result, "a".to_string());
/// let mut steps_iter = vec![RegexStep { rep: RegexRep::Exact(1), val: RegexValue::Literal('a') }].iter();
/// let mut input_chars = "b".chars();
/// let mut match_state = MatchState::InProgress;
/// let result = "".to_string();
/// process_regex_steps(&mut steps_iter, &mut input_chars, &mut match_state, &mut result);
/// assert_eq!(result, "".to_string());
pub fn process_regex_steps(
    steps_iter: &mut std::slice::Iter<RegexStep>,
    input_chars: &mut std::str::Chars,
    match_state: &mut MatchState,
    result: &mut String,
) {
    while *match_state == MatchState::InProgress {
        let step = match steps_iter.next() {
            Some(s) => s,
            None => {
                *match_state = MatchState::EndOfRegex;
                break;
            }
        };

        let actual_char = match input_chars.next() {
            Some(c) => c,
            None => {
                *match_state = MatchState::EndOfWord;
                break;
            }
        };

        if !handle_step_rep(
            step,
            steps_iter,
            actual_char,
            match_state,
            input_chars,
            result,
        ) {
            break;
        }
    }
}

/// Compares a regex with a word
/// # Arguments
/// * `regex` - A reference to a Regex
/// * `word` - A reference to a str
/// # Returns
/// * A String
/// # Example
/// let regex = Regex::new("a").unwrap();
/// let word = "a";
/// let result = compare_regex_with_expression(&regex, &word);
/// assert_eq!(result, "a".to_string());
/// let regex = Regex::new("a").unwrap();
/// let word = "b";
/// let result = compare_regex_with_expression(&regex, &word);
/// assert_eq!(result, "".to_string());
pub fn compare_regex_with_expression(regex: &Regex, word: &str) -> String {
    let mut result = String::new();
    let mut match_state: MatchState = MatchState::InProgress;
    let mut steps_iter = regex.steps.iter();
    let mut input_chars = word.chars();

    process_regex_steps(
        &mut steps_iter,
        &mut input_chars,
        &mut match_state,
        &mut result,
    );

    handle_end_of_expression(match_state, regex, word, &mut input_chars, result)
}

/// Compares the regexes with a word
/// - It returns the matched part of the word
/// - If the orginial expression had "|", it would receive more than one regex
/// # Arguments
/// * `regexes` - A reference to a vector of Regex
/// * `s` - A String
/// # Returns
/// * A Result with a String or an error
/// # Example
/// let regexes = vec![Regex::new("a").unwrap()];
/// let s = "a".to_string();
/// let result = compare_regexes_with_expression(&regexes, s);
/// assert_eq!(result, Ok("a".to_string()));
/// let regexes = vec![Regex::new("a").unwrap(), Regex::new("b").unwrap()];
/// let s = "b".to_string();
/// let result = compare_regexes_with_expression(&regexes, s);
/// assert_eq!(result, Ok("b".to_string()));
pub fn compare_regexes_with_expression(
    regexes: &Vec<Regex>,
    s: String,
) -> Result<String, &'static str> {
    for regex in regexes {
        let result = compare_regex_with_expression(regex, &s);
        if !result.is_empty() {
            return Ok(s);
        }
    }

    Err("No match found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex;

    mod exact {
        use super::*;

        mod literal {
            use super::*;

            #[test]
            fn test_1() {
                let regex = regex::Regex::new("a").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_2() {
                let regex = regex::Regex::new(".").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_3() {
                let regex = regex::Regex::new("a").unwrap();
                let word = "b".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_4() {
                let regex = regex::Regex::new("a").unwrap();
                let word = "ab".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
            }

            #[test]
            fn test_5() {
                let regex = regex::Regex::new("a").unwrap();
                let word = "ba".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
            }

            #[test]
            fn test_6() {
                let regex = regex::Regex::new("a").unwrap();
                let word = "bab".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
            }
        }

        mod wildcard {
            use super::*;

            #[test]
            fn test_1() {
                let regex = regex::Regex::new(".").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_2() {
                let regex = regex::Regex::new(".").unwrap();
                let word = "ab".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
            }

            #[test]
            fn test_3() {
                let regex = regex::Regex::new(".").unwrap();
                let word = "ba".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "b".to_string()
                );
            }

            #[test]
            fn test_4() {
                let regex = regex::Regex::new(".").unwrap();
                let word = "bab".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "b".to_string()
                );
            }
        }

        mod classes {
            use super::*;
            #[test]
            fn test_alpha() {
                let regex = regex::Regex::new("[[:alpha:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_alpha_f() {
                let regex = regex::Regex::new("[[:alpha:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_alnum() {
                let regex = regex::Regex::new("[[:alnum:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_alnum_f() {
                let regex = regex::Regex::new("[[:alnum:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_digit() {
                let regex = regex::Regex::new("[[:digit:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_digit_f() {
                let regex = regex::Regex::new("[[:digit:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_lower() {
                let regex = regex::Regex::new("[[:lower:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_lower_f() {
                let regex = regex::Regex::new("[[:lower:]]").unwrap();
                let word = "A".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_upper() {
                let regex = regex::Regex::new("[[:upper:]]").unwrap();
                let word = "A".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_upper_f() {
                let regex = regex::Regex::new("[[:upper:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_punct() {
                let regex = regex::Regex::new("[[:punct:]]").unwrap();
                let word = "!".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_punct_f() {
                let regex = regex::Regex::new("[[:punct:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_space() {
                let regex = regex::Regex::new("[[:space:]]").unwrap();
                let word = " ".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_space_f() {
                let regex = regex::Regex::new("[[:space:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }
        }

        mod oneof {
            use super::*;

            #[test]
            fn test_1() {
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_2() {
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "b".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_3() {
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "c".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), word);
            }

            #[test]
            fn test_4() {
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "d".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn test_5() {
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "ab".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
            }

            #[test]
            fn test_6() {
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "ba".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "b".to_string()
                );
            }

            #[test]
            fn test_7() {
                let regex = regex::Regex::new("[abc]").unwrap();
                let word = "bab".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "b".to_string()
                );
            }
        }
    }

    mod none {
        use crate::regex;

        use super::*;

        #[test]
        fn vocal() {
            let regex = regex::Regex::new("[^aeiou]").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "c".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn literals() {
            let regex = regex::Regex::new("[^abc]").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "d".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        mod classes {
            use super::*;
            #[test]
            fn alpha() {
                let regex = regex::Regex::new("[^[:alpha:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "1".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "1".to_string()
                );
                let word = "%".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "%".to_string()
                );
            }

            #[test]
            fn alnum() {
                let regex = regex::Regex::new("[^[:alnum:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "%".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "%".to_string()
                );
            }

            #[test]
            fn digit() {
                let regex = regex::Regex::new("[^[:digit:]]").unwrap();
                let word = "1".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
                let word = "%".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "%".to_string()
                );
            }

            #[test]
            fn lower() {
                let regex = regex::Regex::new("[^[:lower:]]").unwrap();
                let word = "a".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "A".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "A".to_string()
                );
                let word = "%".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "%".to_string()
                );
            }

            #[test]
            fn upper() {
                let regex = regex::Regex::new("[^[:upper:]]").unwrap();
                let word = "A".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
                let word = "%".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "%".to_string()
                );
            }

            #[test]
            fn punct() {
                let regex = regex::Regex::new("[^[:punct:]]").unwrap();
                let word = "!".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
                let word = "%".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            }

            #[test]
            fn space() {
                let regex = regex::Regex::new("[^[:space:]]").unwrap();
                let word = " ".to_string();
                assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
                let word = "a".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "a".to_string()
                );
                let word = "%".to_string();
                assert_eq!(
                    compare_regex_with_expression(&regex, &word),
                    "%".to_string()
                );
            }
        }
    }

    mod range {
        use crate::regex;

        use super::*;

        #[test]
        fn test_1() {
            let regex = regex::Regex::new("a{2}").unwrap();
            let word = "aa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "aaa".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "aa".to_string()
            );
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "aab".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "aa".to_string()
            );
        }

        #[test]
        fn test_2() {
            let regex = regex::Regex::new("a{2,}").unwrap();
            let word = "aa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "aaa".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "aaa".to_string()
            );
        }

        #[test]
        fn test_3() {
            let regex = regex::Regex::new("a{2,3}").unwrap();
            let word = "aa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "aaa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "aaaa".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "aaa".to_string()
            );
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }

        #[test]
        fn test_4() {
            let regex = regex::Regex::new("a{2,4}b").unwrap();
            let word = "aa".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "aab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "aaab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "aaaab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn test_5() {
            let regex = regex::Regex::new("a+b").unwrap();
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "aab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "aaab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }
    }

    mod start_of_line {
        use super::*;

        #[test]
        fn test_1() {
            let regex = regex::Regex::new("^a").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn test_2() {
            let regex = regex::Regex::new("^a").unwrap();
            let word = "ba".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }

        #[test]
        fn test_3() {
            let regex = regex::Regex::new("^[aeiou]").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "ba".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "b".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }
    }

    mod end_of_line {
        use super::*;

        #[test]
        fn test_1() {
            let regex = regex::Regex::new("a$").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
        }

        #[test]
        fn test_2() {
            let regex = regex::Regex::new("a$").unwrap();
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }

        #[test]
        fn test_3() {
            let regex = regex::Regex::new("[aeiou]$").unwrap();
            let word = "a".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "ba".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "a".to_string()
            );
            let word = "ab".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "b".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }
    }

    mod mandatory {
        use super::*;

        #[test]
        fn test_1() {
            let regex = regex::Regex::new("ab.cd").unwrap();
            let word = "abcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "abxcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "xabxcd".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "abxcd".to_string()
            );
            let word = "abxcdx".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "abxcd".to_string()
            );
        }

        #[test]
        fn test_2() {
            let regex = regex::Regex::new("ab.*cd").unwrap();
            /* let word = "abcd".to_string();
            assert_eq!(
                compare_regex_with_expression(&regex, &word),
                "abcd".to_string()
            );*/
            let word = "abxcd".to_string();
            assert!(!compare_regex_with_expression(&regex, &word).is_empty());
            let word = "abxcdx".to_string();
            assert!(!compare_regex_with_expression(&regex, &word).is_empty());
            let word = "xabxcdx".to_string();
            assert!(!compare_regex_with_expression(&regex, &word).is_empty());
            let word = "abxxcd".to_string();
            assert!(!compare_regex_with_expression(&regex, &word).is_empty());
        }

        #[test]
        fn test_3() {
            let regex = regex::Regex::new("a[bc]d").unwrap();
            let word = "abcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "abd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "acd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "axd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }

        #[test]
        fn test_4() {
            let regex = regex::Regex::new("ab{2,4}cd").unwrap();
            let word = "abcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
            let word = "abbcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "abbbcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "abbbbcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), word);
            let word = "abbbbbbcd".to_string();
            assert_eq!(compare_regex_with_expression(&regex, &word), "".to_string());
        }
    }
}
