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
    /* ah... split off, thou art a heartless bitch */

    let params = Parameters::parse_argv(argv).unwrap_or_else(|err| {
	println!("could not parse parameters:{err}");
	process::exit(1);
    });

    let cts:Counts = counts::all_files_counts(params.files.into_iter());

    display::print_counts(params.display_flags, cts);
}

