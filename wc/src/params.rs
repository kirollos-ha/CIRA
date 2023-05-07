/* we give argv as a vector
 * the files list given is going to be
 * an iterator over some of its members
 * so
 */
use std::slice::Iter;
use std::iter::Skip;

use crate::display;
use crate::display::Flags as DisplayFlags;

#[derive(Debug)]
pub struct Parameters<'a> {
    pub display_flags : DisplayFlags,
    pub files : Skip<Iter<'a,String>>
}

impl Parameters<'_> {
    pub fn parse_argv(argv:&Vec<String>) -> Result<Parameters ,&str> {
	if argv.len() < 2 {
	    return Err("not enough arguments");
	}
	let gave_flags:bool = display::string_is_flags(&argv[1]);
	/* declare flaggy flaggy with the ressy ressy */
	Ok(Parameters {
	    // my_name : my_name,
	    display_flags : {
		if gave_flags { DisplayFlags::from(&argv[1]) }
		else { DisplayFlags::default() }
	    },
	    files : {
		if gave_flags { argv.iter().skip(2).into_iter() }
		else { argv.iter().skip(1).into_iter() }
	    },
	})
    }
}

/* tests, to be separated later */
#[cfg(test)]
mod tests {
    #[test]
    fn not_enough() {
	match super::Parameters::parse_argv(&vec![String::from("a")]) {
	    Ok(_) => panic!("parameters should not have enough args"),
	    Err(s) => assert_eq!(s,"not enough arguments"),
	}
    }
}
