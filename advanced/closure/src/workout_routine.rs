use std::thread;
use std::time::Duration;

pub fn workout(intensity: i32, random_number: i32) {
    let action = || {
        println!("start...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("do some push-ups {} {}", action(), random_number);
    } else if random_number == 3 {
        println!("Take a rest...");
    } else {
        println!("Let's end the workout, it's exhausting... {}", action());
    }
}