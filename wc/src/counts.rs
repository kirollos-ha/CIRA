/* all file opfn test_istrugening currently going on here */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/* "adapted" from rust by example */
fn read_lines<P : AsRef<Path>>(filename: P)
			       -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[derive(Debug)]
pub struct Counts {
    pub chars: u32,
    pub words: u32,
    pub lines: u32,
    pub files: u32,
}

#[allow(dead_code)]
fn do_lines<T,F>(lines:T, fun:F) where T:Iterator<Item = io::Result<String>>,
				       F:Fn(String) -> ()
{
    for(line_number, line) in lines.enumerate() {
	match line {
	    Ok(line) => fun(line),
	    Err(problem) => panic!("{problem} at line {line_number}"),
	}
    }
}

#[allow(dead_code)]
fn do_file<F>(file_name:String, fun:F) where F:Fn(String) -> () {
    if let Ok(lines) = read_lines(file_name) {
	do_lines(lines, fun);
    }
}

impl Counts {
    pub fn single_file(file_name:&String) -> Result<Counts, String> {
	/* adapted from rust by example */
	let mut accum = Self::all_zeros();
	accum.files = 1;

	/* TODO: encapsulate this block in a do_lines macro/higher ord. function */
	if let Ok(lines) = read_lines(file_name) {
	    for (line_ind, line) in lines.enumerate() {
		match line {
		    Ok(line) => accum.add_line_counts(&line),
		    Err(problem) =>  {
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
    pub fn all_zeros() -> Counts {
	Self {
	    chars: 0,
	    words: 0,
	    lines: 0,
	    files: 0,
	}
    }
    fn words_in_line(line:&String) -> u32 {
	/* I don't know how else to get the first char, this is NOT ideal */
	/*
	let mut in_word: bool = !(line
				  .chars()
				  .next()
				  .expect("failed to read line")
				  .is_whitespace());
	 */
	if line.len() == 0 {
	    return 0;
	}
	let mut in_word:bool = !(line.as_bytes()[0].is_ascii_whitespace());
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

    
