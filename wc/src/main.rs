/* this wc is currently mildly wack */

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::slice::Iter;
use std::iter::Skip;

use std::env;

use std::process;

fn main() {
    let mut chars:u32 = 0;
    let mut words:u32 = 0;
    let mut lines:u32 = 0;
    let mut files:u32 = 0;

    let argv:Vec<String> = env::args().collect();
    /* per ora si assume che l'input vada bene */

    let params = Parameters::from(&argv).unwrap_or_else(|err| {
	println!("could not parse parameters:{err}");
	process::exit(1);
    });
    for file_name in params.files {
	let counts = match Counts::count_in_file(&file_name) {
	    Ok(counts) => counts,
	    Err(problem) => {
		println!("skipping count for file : {}, {}", file_name, problem);
		continue
	    }
	};
	files += 1;
	chars += counts.chars;
	words += counts.words;
	lines += counts.lines;
    }
    println!("{} files\n{} chars\n{} words\n{} lines",files, chars, words, lines);
}

#[derive(Debug)]
struct Parameters<'a> {
    my_name : String,
    flags : DisplayFlags,
    files : Skip<Iter<'a,String>>
}

#[derive(Debug)]
struct DisplayFlags {
    display_chars: bool,
    display_words: bool,
    display_lines: bool,
    display_files: bool,
}

impl Parameters<'_> {
                                  /* sospetto che Self non vada bene */
    fn from(argv:&Vec<String>) -> Result<Parameters ,&str> {
	let my_name = argv[0].clone();
	if argv.len() < 2 {
	    return Err("not enough arguments");
	}
	let gave_flags:bool = DisplayFlags::string_is_flags(&argv[1]);
	/* declare flaggy flaggy with the ressy ressy */
	Ok(Parameters {
	    my_name : my_name,
	    flags : {
		if gave_flags { DisplayFlags::from(&argv[1]) }
		else { DisplayFlags::all_true_flags() }
	    },
	    files : {
		if gave_flags { argv.iter().skip(2).into_iter() }
		else { argv.iter().skip(1).into_iter() }
	    },
	})
    }
}

impl DisplayFlags {
    fn string_is_flags(s:&String) -> bool {
	s.len()>1 && (s.as_bytes()[0] == b'-')
    }
    fn from(s:&String) -> DisplayFlags {
	assert_eq!(DisplayFlags::string_is_flags(s),true);
	let mut res = Self {
	    display_chars : false,
	    display_words : false,
	    display_lines : false,
	    display_files : false,
	};
	for c in s.chars().skip(1) {
	    match c {
		'c' => res.display_chars = true,
		'w' => res.display_words = true,
		'l' => res.display_lines = true,
		'f' => res.display_files = true,
		_ => {
		    println!("invalid flag : {}, defaulting to all true", c);
		    return Self::all_true_flags()
		},
	    }
	};
	res
    }
    fn all_true_flags() -> Self {
	Self {
	    display_chars : true,
	    display_words : true,
	    display_lines : true,
	    display_files : true,
	}
    }
}

struct Counts {
    chars: u32,
    lines: u32,
    words: u32,
}

impl Counts {
    fn count_in_file(file_name:&String) -> Result<Counts, String> {
	/* adapted from rust by example */
	let mut accum = Counts::all_zeros();
	if let Ok(lines) = read_lines(file_name) {
	    for (line_ind, line) in lines.enumerate() {
		match line {
		    Ok(line) => accum.add_line_counts(&line),
		    Err(problem) => {
			return Err(format!("{problem} in line {line_ind}"));
		    }
		}
	    }
	}
	Ok(accum)
    }
    fn add_line_counts(&mut self, line:&String) {
	self.lines += 1;
	self.chars += 1 + line.len() as u32;
	/* 1 + for the newline char
	 * (not quite ideal or cross platform) */
	self.words += Self::words_in_line(line);
    }
    fn all_zeros() -> Counts {
	Self {
	    chars: 0,
	    words: 0,
	    lines: 0,
	}
    }
    fn words_in_line(line:&String) -> u32 {
	/* I don't know how else to get the first char, this is NOT ideal */
	let mut in_word: bool = !(line.chars().next().expect("").is_whitespace()); 
	let mut cnt:u32 = 0;
	for char in line.chars() {
	    if char.is_whitespace() {
		if in_word {
		    cnt += 1;
		    in_word = false;
		}
	    }
	    else {
		in_word = true;
	    }
	}
	/* since final newlines are excluded
	 * the last word might not be counted
	 * unless explicitly counted
	 */
	if in_word { cnt += 1; }
	cnt
    }
}

/* "adapted" from rust by example */
fn read_lines<P : AsRef<Path>>(filename: P)
			       -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
