fn main() {

    let elem = 12u8;
    let elem1 = 22u8;
    let elem2 = 220u8;
    let mut vec = Vec::new();

    vec.push(elem1);
    vec.push(elem2);
    vec.push(elem);

    println!("{:?}", vec);
}
