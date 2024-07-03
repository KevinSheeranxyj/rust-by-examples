
struct Point {
    x: u32,
    y: u32
}


struct Person {
    name: String,
    age: u8
}

struct Solidity {

}

struct Rust {

}

struct Web3 {
    ethereum: Solidity,
    solana: Rust
}


struct Unit;

struct Pair(i32, i32);

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("None: {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
