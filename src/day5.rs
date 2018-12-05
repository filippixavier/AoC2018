extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use self::regex::Regex;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
	let alpha = Regex::new(r"aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg|hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn|oO|Oo|pP|Pp|qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu|vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz").unwrap(); 
	let file = fs::read_to_string(Path::new("./data/day5.txt"))?;
	let mut input = file;

	input = remove_all(input, &alpha);
	println!("Non repeating polymer length:\n{}", input.len());
	Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
	let alpha = Regex::new(r"aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg|hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn|oO|Oo|pP|Pp|qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu|vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz").unwrap(); 
	let alphabet = "abcdefghijklmnopqrstuvwxyz".split("");
	let file = fs::read_to_string(Path::new("./data/day5.txt"))?;
	let input = file;
	let mut min_size = input.len();
	let mut letter_to_remove = "";
	for letter in alphabet {
		let remove_letter = Regex::new(format!(r"(?i){}", letter).as_str()).unwrap();
		let sub_input = remove_all(remove_letter.replace_all(input.clone().as_str(), "").into_owned(), &alpha);
		min_size = if sub_input.len() < min_size {
			letter_to_remove = letter;
			sub_input.len()
		} else {
			min_size
		}
	};

	println!("Smallest size: {} with letter {} removed", min_size, letter_to_remove);

	Ok(())
}

fn remove_all(mut input: String, reg: &Regex) -> String {
	while reg.is_match(input.as_str()) {
		input = reg.replace_all(input.as_str(), "").into_owned();
	}
	input
}