

trait MyType {

}

impl<T> From<T> for MyType {

}

impl<T> From<T> for MyType<T> {

}


impl<T> From<MyType> for Vec<T> {

}


impl<T> ForeignTrait<MyType, T> for Vec<T> {

}