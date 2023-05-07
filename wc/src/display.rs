/* display module */

use crate::counts;

pub fn string_is_flags(s:&String) -> bool {
    s.len()>1 && (s.as_bytes()[0] == b'-')
}

pub fn print_counts(df:Flags, cnts: counts::Counts) {
    if df.display_chars {
	println!("{} chars", cnts.chars);
    }
    if df.display_words {
	println!("{} words", cnts.words);
    }
    if df.display_lines {
	println!("{} lines", cnts.lines);
    }
}

#[derive(Debug)]
pub struct Flags {
    display_chars: bool,
    display_words: bool,
    display_lines: bool,
    display_files: bool,
}

impl Flags {
    pub fn from(s:&String) -> Flags {
	assert_eq!(string_is_flags(s),true);
	let mut res = Self::all_false();
	for c in s.chars().skip(1) { // skip the first hyphen
	    match c {
		'c' => res.display_chars = true,
		'w' => res.display_words = true,
		'l' => res.display_lines = true,
		'f' => res.display_files = true,
		_ => println!("invalid flag : {}, not considering it", c),
	    }
	};
	res
    }
    pub fn default() -> Self {
	Self::all_true()
    }
    fn all_true() -> Self {
	Self {
	    display_chars : true,
	    display_words : true,
	    display_lines : true,
	    display_files : true,
	}
    }

    fn all_false() -> Self {
	Self {
	    display_chars : false,
	    display_words : false,
	    display_lines : false,
	    display_files : false,
	}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_flags() {
	assert!(string_is_flags(&String::from("-w")));
	assert!(string_is_flags(&String::from("-c")));
	assert!(string_is_flags(&String::from("-l")));
    }
    #[test]
    fn test_is_not_flags() {
	assert!(!string_is_flags(&String::from("-")));
	assert!(!string_is_flags(&String::from("")));
	assert!(!string_is_flags(&String::from("w")));
	assert!(!string_is_flags(&String::from("w-")));
    }
    #[test]
    #[should_panic]
    fn test_constructing_with_invalid_flags() {
	Flags::from(&String::from("wcl"));
    }
    #[test]
    fn test_valid_options() {
	let f=Flags::from(&String::from("-wc"));
	assert!(f.display_words);
	assert!(f.display_chars);
	assert!(!(f.display_lines));
	assert!(!(f.display_files));
    }
    #[test]
    fn test_invalid_options() {
	let f=Flags::from(&String::from("-Wrl"));
	assert!(! f.display_words);
	assert!(! f.display_chars);
	assert!((f.display_lines));
	assert!(! (f.display_files));
    }
}
