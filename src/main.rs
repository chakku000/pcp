use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    let mut paths: Vec<PathBuf> = Vec::new();

    for input_path in args {
        let path = Path::new(input_path);
        if path.is_file() {
            paths.push(path.to_path_buf());
        } else if path.is_dir() {
            for entry in WalkDir::new(input_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_dir() { continue }
                paths.push(entry.path().to_path_buf());
            }
        } else {
            panic!("{} not found", input_path)
        }
    }

    for path in paths{
        let path_string = path.to_str().unwrap();
        let mut f = File::open(path_string).expect("file not found");
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(_) => {
                continue
            }
        }
        println!("```{}\n{}\n```", path_string, contents);
    }
}
