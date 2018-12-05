extern crate regex;

use std::error::Error;
use std::fs;
use std::path::Path;

use self::regex::Regex;

pub fn first_star() -> Result<(), Box<Error + 'static>> {
	let alpha = Regex::new(r"aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg|hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn|oO|Oo|pP|Pp|qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu|vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz").unwrap(); 
	let file = fs::read_to_string(Path::new("./data/day5.txt"))?;
	let mut input = file;

	while alpha.is_match(input.as_str()) {
		input = alpha.replace_all(input.as_str(), "").into_owned();
	}
	println!("Non repeating polymer length:\n{}", input.len());
	Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
	Ok(())
}