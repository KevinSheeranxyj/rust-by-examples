use std::path::Path;

pub fn print_path<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    println!("Path: {:?}", path);
}
