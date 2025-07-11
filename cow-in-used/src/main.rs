
use std::borrow::Cow;

fn greet(name: Cow<str>) {
    println!("Name: {}", name);
}

fn main() {
    let alice = "Alice"; // borrowed
    let jack = String::from("Jack"); // owned

    // greet(Cow::Borrowed(alice));
    // greet(Cow::Owned(jack));

    let update = "lsing";

    String::from(&update);
}
