use get_random_const::random;
use closure::workout;

fn main() {
    let random_number = random!(i32);
    workout(20, random_number);

}
