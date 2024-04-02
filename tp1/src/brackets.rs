use crate::regex::RegexRep;
use crate::regex::RegexStep;
use crate::regex::RegexValue;
use std::str::Chars;

pub fn handle_brackets(chars_iter: &mut Chars, steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, &'static str> {
    /* Matching a Single Character:
	Regex: egrep '[abc]'
	Input: abcde
	Result: Matches any character in the set {a, b, c}. Output: a
	Matching a Range of Characters:

	Regex: egrep '[a-z]'
	Input: Hello World
	Result: Matches any lowercase letter. Output: elloorld
	Matching Multiple Characters:

	Regex: egrep '[aeiou]'
	Input: Hello World
	Result: Matches any vowel. Output: eoo
	Excluding Characters:

	Regex: egrep '[^0-9]'
	Input: abc123def
	Result: Matches any character that is not a digit. Output: abcdef
	Matching Alphanumeric Characters:

	Regex: egrep '[[:alnum:]]'
	Input: abc123!@#
	Result: Matches any alphanumeric character. Output: abc123
	Matching Uppercase Letters:

	Regex: egrep '[[:upper:]]'
	Input: Hello World
	Result: Matches any uppercase letter. Output: H W
	Matching Spaces:

	Regex: egrep '[[:space:]]'
	Input: Hello World
	Result: Matches any whitespace character. Output:
	Matching Punctuation Characters:

	Regex: egrep '[[:punct:]]'
	Input: Hello, World!
	Result: Matches any punctuation character. Output: , ! */
	let mut n = String::new();
	for c in chars_iter.by_ref() {
		if c == ']' {
			break;
		}
		n.push(c);
	}
	if n.is_empty() {
		return Err("Empty brackets");
	}


	let mut chars = n.chars();
	
	return Ok(Some(RegexStep{
		rep: RegexRep::Exact(1), 
		val: RegexValue::Optional(chars.collect())
	}));


}