use std::path::Path;
use reference::{MyStruct, print_length, print_path};


fn main() {
    let path_str = "/https/some/path";
    let path_buff = Path::new(path_str).to_path_buf();

    print_path(path_str);
    print_path(path_buff);

    let str = String::from("hello world");
    let str_slice = "Hello rust";
    print_length(str);
    print_length(str_slice);

    let my_struct = MyStruct {
        value: String::from("Hello world")
    };

    println!("=====================");

    let my_str: &str = my_struct.as_ref();
    println!("MyStruct as str: {}", my_str);

}
