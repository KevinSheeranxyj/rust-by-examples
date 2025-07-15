use std::cell::RefCell;
use std::rc::Rc;

// Example with Rc<RefCell<T>> for Mutability
pub fn test() {
    let shared = Rc::new(RefCell::new(5));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    *a.borrow_mut() += 10;
    *b.borrow_mut() *= 20;

    println!("Final value: {}", shared.borrow());
}