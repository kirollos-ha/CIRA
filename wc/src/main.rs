/* this wc is currently mildly wack */


use std::env;

use std::process;

mod display;
mod counts;
mod params; /* the parameter parsing logic is a bit too "funky" */

use counts::Counts;
use params::Parameters;


fn main() {

    let argv:Vec<String> = env::args().collect();

    let params = Parameters::parse_argv(&argv).unwrap_or_else(|err| {
	println!("could not parse parameters:{err}");
	process::exit(1);
    });

    /* currently can't move out of main due to some weird dependencies on the iteration
     * need to refactor the iteration out of params::Parameters
     * before moving this block can be moved to a function in the counts module
     * as it's still overly dependant on params::Parameters, and it doesn't make sense
     * to couple the counting with the parameter formatting
     */
    let mut all_files_counts = Counts::all_zeros();
    // fixme(?): dependency on the internal structure of params
    for file_name in params.files {
	let file_counts = match Counts::single_file(&file_name) {
	    Ok(counts) => counts,
	    Err(problem) => {
		println!("skipping count for file : {}, {}", file_name, problem);
		continue
	    }
	};

	/* TODO implement Add trait for Counts */
	all_files_counts.files += 1;
	all_files_counts.chars += file_counts.chars;
	all_files_counts.words += file_counts.words;
	all_files_counts.lines += file_counts.lines;
    }


    display::print_counts(params.display_flags, all_files_counts);
}
