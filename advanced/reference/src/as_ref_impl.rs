

pub struct MyStruct {
    pub value: String
}


impl AsRef<str> for MyStruct {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

