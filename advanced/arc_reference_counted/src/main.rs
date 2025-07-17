mod advanced_test;

use std::sync::Arc;
use std::thread;

//✅ Use Arc<T> when:
// 	•	Multiple threads need to access the same data, and that data is read-only or read-write with synchronization.
// 	•	You need shared ownership: multiple owners that can live independently.
// 	•	You’re in a multi-threaded context, unlike Rc<T> which is for single-threaded scenarios.

// ❌ Don’t use Arc<T> when:
//	•	You’re working in a single-threaded context — prefer Rc<T> then.
// 	•	You only need one owner — prefer Box<T> or just the value itself.
// 	•	You don’t want the performance cost of atomic reference counting (it’s slower than Rc<T> due to atomic operations).

// Sharing data across threads
fn main() {

    let numbers = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];

    for _ in 0..3 {
        let numbers_clone = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            println!("{:?} ", numbers_clone)
        });
        handles.push(handle)
    }

}
