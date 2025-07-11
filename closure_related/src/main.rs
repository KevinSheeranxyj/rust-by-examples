
fn main() {

    let mut a = "solana";

    let b = String::from("sing");

    let lenth = first_word(&b);
    
}

fn first_word(name: &String) -> usize {
    
    let bytes = name.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    name.len()
    
}
