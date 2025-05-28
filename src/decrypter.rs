use std::path::{Path, PathBuf};

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
	println!("Not implemented yet, but got file {}, and the stripped filename is {}", file, stripped_file);
}