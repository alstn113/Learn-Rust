// downcast는 보통 universal logger에 사용됨
// struct MyLogger(Vec<&dyn Any>)

use std::any::{type_name, Any};

// typename
fn get_typename<T: Any>(_: T) {
    let typename = type_name::<T>();
    println!("{}", typename);
}

// is
fn is_trait(input: &dyn Any) {
    if input.is::<String>() {
        println!("We got a String")
    } else if input.is::<i32>() {
        println!("We got an i32")
    } else {
        println!("We got something else")
    }
}

// downcast
fn try_do_get_string(input: &dyn Any) {
    if let Some(a_string) = input.downcast_ref::<String>() {
        println!("We have a String {a_string}")
    } else {
        println!("We don't have a String")
    }
}

fn main() {
    // get_typename
    get_typename(8);
    get_typename("sdf");
    get_typename("sdf".to_string());
    get_typename(vec!["sdf", "sdf"]);
    println!("");

    // is trait
    is_trait(&"sdf");
    is_trait(&8);
    is_trait(&vec!["sdf", "sdf"]);
    println!("");

    // downcast trait
    try_do_get_string(&9);
    try_do_get_string(&"Hello World");
    try_do_get_string(&String::from("Hello World"));
}
