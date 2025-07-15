use std::cell::RefCell;
// Important:
// you can have multiple immutable borrows( borrow() )
// you can have one mutable borrow( borrow_mut() )
pub fn test() {
    let data = RefCell::new(5);
    println!("before: {}", data.borrow());

    *data.borrow_mut() = 10;
    println!("after: {}", data.borrow());
    
}