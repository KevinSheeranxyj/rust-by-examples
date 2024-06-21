
struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}


fn main() {
    let age = Years(19);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult？ {}", is_adult(&age_days.to_years()));
}
