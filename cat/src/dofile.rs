use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process;

/* BufRead è un trait
 * BufReader è un tipo che lo implementa
 * ma vaffanculo
 */

/* gli utilizzi di &str sono per il filename
 * gli utilizzi di String sono per le stringhe lette dal buffer
 */
type LineBuf = io::Lines<io::BufReader<File>>;

fn read_lines(filename: &str) -> io::Result<LineBuf> {
    match File::open(filename) {
	Ok(file) => Ok(io::BufReader::new(file).lines()),
	Err(problem) => panic!("cannot open file \"{}\" : {}", filename, problem),
    }
}

/* al di fuori di do_file le stringhe dell'iteratore non esistono
 * quindi sia pure l'ownership alle subroutine
 * se muore prima meglio
 */
fn do_file_lines<F>(lines:LineBuf, fun:F) where F:Fn(String) -> () {
    for(line_number, line) in lines.enumerate() {
	match line {
	    Ok(line) => fun(line),
	    Err(problem) => panic!("{problem} at line {line_number}"),
	}
    }
}

pub fn do_file<F>(file_name:&str, fun:F) where F:Fn(String) -> () {
    let lines = read_lines(file_name).unwrap_or_else(
	|err| {
	    println!("cannot open file {} : {}", file_name, err);
	    process::exit(1);
	});
    do_file_lines(lines, fun);
}


