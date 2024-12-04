use std::path::Path;
use std::fs::{read, write};

/// Read a file in one go
pub fn read_file<P: AsRef<str>>(file: P) -> String {
    let p = Path::new(file.as_ref());
    if !p.is_file() {
        panic!("Not a file: {0}", file.as_ref());
    }

    String::from_utf8(read(p).expect("Should have been a readable file")).unwrap()
}

/// Read a file line by line
pub fn read_lines<P: AsRef<str>>(file: P) -> Vec<String> {
    read_file(file).split("\n").map(ToString::to_string).collect()
}

pub fn vec2str<T: ToString>(v: &Vec<T>, sep: &str) -> String {
    let mut s = String::new();
    for i in 0..v.len() {
        s.push_str(v[i].to_string().as_str());
        if i + 1 < v.len() {
            s += sep;
        }
    }
    s
}

pub fn to_file<P: AsRef<str>>(p: P, content: String) {
    let path = Path::new(p.as_ref());
    write(path, content).unwrap()
}

/// Make a rows x cols matrix
pub fn make_matrix<T: Default>(rows: usize, cols: usize) -> Vec<Vec<T>> {
    let mut mat = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut col = Vec::with_capacity(cols);

        for _ in 0..cols {
            col.push(T::default());
        }

        mat.push(col)
    }
    mat
}
