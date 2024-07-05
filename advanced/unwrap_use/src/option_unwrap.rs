

pub fn check_with_none() {

    let some_value = Some(432);
    let none_value: Option<i32> = None;

    // this will successfully unwrap and print the value
    println!("This value is: {}", some_value.unwrap());

    // This will cause a panic bacause this is None
    // println!("The value is: {}", none_value.unwrap());
}