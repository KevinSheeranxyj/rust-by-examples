use crate::print_path;

pub fn check_with_result() {
    println!("{:?}", 12928);
}

pub fn print_length<T: AsRef<str>>(words: T) {
    let str = words.as_ref();
    println!("the length of string: {}", str.len());
}