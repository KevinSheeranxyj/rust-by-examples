pub fn create_vector() {
    let mut vec = vec![12, 9];
    vec.push(1);

    for i in &mut vec {
        println!("{i}");
    }
}


