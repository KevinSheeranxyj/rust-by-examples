mod test01;
mod test02;
mod test03;

use std::rc::Rc;

// Limitation:
// 	•	Rc<T> is not thread-safe. For multi-threading, use Arc<T>.
// 	•	You cannot mutate the data directly through Rc<T> unless you also wrap it in something like RefCell<T>.
fn main() {


    let data = Rc::new(String::from("hello"));

    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);

    println!("data: {}", data);
    println!("data1: {}", data1);
    println!("data2: {}", data2);

    println!("Show reference counter: {}", Rc::strong_count(&data));

    test01::test();
    test02::test();

}
