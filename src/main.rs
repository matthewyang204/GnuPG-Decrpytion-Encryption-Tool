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

fn version() {
	println!("GnuPG Decryption and Encryption Tool, version 1.0.1");
	println!("(C) 2025 Matthew Yang");
}

fn help() {
    version();
	println!("");
	println!("Usage: gpgde -d|-e filename");
	println!("-d, --decrypt     Decrypt file specified");
	println!("-e, --encrypt     Encrypt file specified");
	println!("filename          Name of file");
}

fn userError() {
    println!("Error: need two args, one with action option -d or -e, and the second is the path to a file");
    exit(1);
}

fn main() {
	let args = getArgs();
	
	if args.len() == 3 {
        if args[1] == "-d" || args[1] == "--decrypt" {
            decrypt(&args[2]);
        } else if args[1] == "-e" || args[1] == "--encrypt" {
            encrypt(&args[2]);
        } else {
            help();
            userError();
        }
	} else if args.len() == 2 {
        if args[1] == "-v" || args[1] == "--version" {
            version();
        } else if args[1] == "-h" || args[1] == "--help"{
            help();
        } else {
            help();
            userError();
        }
    } else {
		help();
        userError();
	}
}
