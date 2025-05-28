use std::env;
use std::process::exit;

fn getArgs() -> Vec<String> {
	let args: Vec<String> = env::args().collect();
	return args;
}

fn help() {
	println!("Usage: gpgde -d|-e filename");
	println!("-d, --decrypt     Decrypt file specified");
	println!("-e, --encrypt     Encrypt file specified");
	println!("filename          Name of file");
}

fn main() {
	let args = getArgs();
	
	if args.len() != 2 {
		help();
		exit(1);
	}
}
