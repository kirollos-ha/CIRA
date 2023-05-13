use crate::dofile::do_file;
use std::iter::Iterator;

pub fn print_files<T:Iterator<Item=String>>(files:T) {
    for file_name in files {
	do_file(&file_name, |line| {
	    println!("{}",line);
	});
    }
}
