// Regular imports
use std::env;
use std::process::exit;

// Load decryption and encryption modules
mod encrypter;
mod decrypter;

// Use functions from loaded modules
use encrypter::encrypt;
use decrypter::decrypt;

fn getArgs() -> Vec<String> {
	let args: Vec<String> = env::args().collect();
	return args;
}

fn help() {
	println!("GnuPG Decryption and Encryption Tool, version 1.0.0");
	println!("(C) 2025 Matthew Yang");
	println!("");
	println!("Usage: gpgde -d|-e filename");
	println!("-d, --decrypt     Decrypt file specified");
	println!("-e, --encrypt     Encrypt file specified");
	println!("filename          Name of file");
}

fn main() {
	let args = getArgs();
	
	if args.len() != 3 {
		help();
		exit(1);
	} else {
		if args[1] == "-d" {
			decrypt(&args[2]);
		} else if args[1] == "-e" {
			encrypt(&args[2]);
		} else {
            help();
        }
	}
}
