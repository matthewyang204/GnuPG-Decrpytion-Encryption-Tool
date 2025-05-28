use std::env;
use std::process::exit;

fn getArgs() {
	let args: Vec<String> = env::args().collect();
	return args;
}

fn help() {
	println!("Usage: gpgde OPTIONS filename");
	println!("-d, --decrypt     Decrypt file specified");
	println!("-e, --encrypt     Encrypt file specified");
	println!("filename          Name of file");
}

fn main() {
	let args = getAgs();
	
	if args.len() != 2 {
		help();
		exit(1);
	}
}
