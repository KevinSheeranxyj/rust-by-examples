use cached::proc_macro::cached;

#[cached]
fn fib(n: u64) -> u64 {
    if n == 0 || n == 1 { return n }
    fib(n-1) + fib(n-2)
}



use std::thread::sleep;
use std::time::Duration;

#[cached(size=100)]
fn keyed(a: String, b: String) -> usize {
    let size = a.len() + b.len();
    sleep(Duration::new(size as u64, 0));
    size
}
//
// #[cached(
//     type = "SizedCache<String, usize>",
//     create = "{ SizedCache::with_size(100) }",
//     convert = r#"{ format!("{}{}", a, b) }"#
// )]
// fn keyed(a: &str, b: &str) -> usize {
//     let size = a.len() + b.len();
//     sleep(Duration::new(size as u64, 0));
//     size
// }


fn main() {
    let fibonacci = fib(12);
    println!("result: {:?}", fibonacci);

   let usize = keyed("22".to_owned(), "v".to_owned());
    println!("size: {:?}", usize);
}
