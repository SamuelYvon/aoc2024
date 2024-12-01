use std::path::Path;
use std::fs::read;

/// Read a file in one go
pub fn read_file<P: AsRef<str>>(file: P) -> String {
   let p = Path::new(file.as_ref());
    if !p.is_file() {
        panic!("Not a file: {0}", file.as_ref());
    }

    String::from_utf8(read(p).expect("Should have been a readable file")).unwrap()
}

/// Read a file line by line
pub fn read_lines<P: AsRef<str>>(file : P) -> Vec<String> {
    read_file(file).split("\n").map(ToString::to_string).collect()
}