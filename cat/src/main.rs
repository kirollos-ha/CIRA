mod printfiles;
mod dofile;

/* &str per i nomi dei file
 * String per ogni cosa che viene iterata senza reference
 * String per ogni cosa che viene iterata
 * String per le righe dei file
 */

fn main() {
    println!("Hello, world!");
    let argv:Vec<String> = std::env::args().collect();
    let p = Params::from(argv);
    printfiles::print_files(p.files.into_iter());
}

struct Params {
    files: Vec<String>,
}

impl Params {
    fn from(mut v:Vec<String>) -> Params {
	Params {
	    files:v.split_off(1),
	}
    }
}
