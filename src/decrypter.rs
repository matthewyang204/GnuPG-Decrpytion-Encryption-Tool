use std::path::{Path, PathBuf};
use std::process::Command;

fn strip_gpg_extension(filename: &str) -> String {
    let path = Path::new(filename);
    if let Some(stem) = path.file_stem() {
        let mut new_path = PathBuf::from(path);
        new_path.set_extension("");
        return new_path.to_string_lossy().to_string();
    }
    filename.to_string()
}

pub fn decrypt(file: &str) {
    let stripped_file = strip_gpg_extension(file);
	
    Command::new("gpg")
    	    .arg("-o")
    	    .arg(stripped_file)
    	    .arg("-d")
    	    .arg(file)
    	    .status()
    	    .expect("Failed to encrypt file");
}