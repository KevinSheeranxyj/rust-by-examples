use get_random_const::random;
use closure::workout;

fn main() {
    let random_number = random!(i32);
    // workout(20, random_number);


    let result = |a: i32, b: i32| { a + b };

    // println!("result: {}", result(2, 222));



    // complicated one

    let mut s = String::new();
    let update_string = |str| s.push_str(str);

    let x = 12;
    let return_number = || x;
    // println!("{}", return_number());
    // exec(update_string);

    // do_twice(return_number());

    println!("{:?}", s);

    let twice = |_| x;
    do_twice_again(twice(1));
    println!("{:?}", do_twice_again(&1));
}


fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

fn do_twice<'a, F>(mut f: F)
    where F: FnMut(&'a i32)
    {
    f()
}

fn do_twice_again<'a, F: FnMut(&'a i32)>(mut f: F) {
    f(&1)
}