
pub fn check_with_result() {
    let ok_value: Result<i32, &str> = Ok(32);
    let error_value: Result<i32, &str> = Err("something went wrong");


    println!("The result is {}", ok_value.unwrap());

}