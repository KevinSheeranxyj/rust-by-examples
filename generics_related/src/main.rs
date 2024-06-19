mod gen_fn;// Non-copyable types
struct Empty;
struct Null;

// A trait generic over `T`
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, _: T) {

    }
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    //empty;
    //null;
    // TODO: Try uncommenting these lines.
}
