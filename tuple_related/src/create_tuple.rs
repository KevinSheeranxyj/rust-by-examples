


pub fn new_tuple() {
    let tup: (i32, f64, u8) = (12, 2.2, 1);

    println!("tuple created :{}, {}, {}", tup.0, tup.1, tup.2);
}

/// tuple example in return parameter
pub fn new_tuple_by_example() {
    let s1 = String::from("hello");

    let(s2, len) = calculate_length(s1);

    println!("The length of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}