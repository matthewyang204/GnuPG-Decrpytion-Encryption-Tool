use std::process::Command;

pub fn encrypt(file: &str) {
    Command::new("gpg")
	    .arg("-c")
	    .arg(file)
	    .status()
	    .expect("Failed to encrypt file");
}