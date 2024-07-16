use std::collections::HashMap;

pub fn get_values() {
    let mut h = HashMap::new();
    h.insert(String::from("Hello"), String::from("World"));

    for (key, value) in h.iter() {
        println!("{} : {}", key, value);
    }
}