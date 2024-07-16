use std::collections::HashMap;

pub fn iterate() {
    let mut v = HashMap::new();
    v.insert(1, "Jane");
    v.insert(2, "Bob");
    v.insert(3, "Maria");

    let keys = v.into_keys();
    for key in keys {
        println!("key: {:?}", key);
    }
}