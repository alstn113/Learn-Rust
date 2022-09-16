use std::any::{type_name, Any};

fn get_typename<T: Any>(_: T) {
    let typename = type_name::<T>();
    println!("{}", typename);
}

fn main() {
    get_typename(8);
    get_typename("sdf");
    get_typename("sdf".to_string());
    get_typename(vec!["sdf", "sdf"]);
}
