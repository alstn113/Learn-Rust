// implemeting a trait for every type that you want to have it
// ex) impl<T> Prints for T {}

use std::fmt::{Debug, Display};

trait Prints {
    fn debug_prints(&self)
    where
        Self: Debug,
    {
        println!("{:?}", self);
    }
    fn display_prints(&self)
    where
        Self: Display,
    {
        println!("{}", self);
    }
}

#[derive(Debug)]
struct Person;

#[derive(Debug)]
struct Building;

impl<T> Prints for T {} // Prints 안에서 Type이 정해져 있음

fn main() {
    let person = Person;
    let building = Building;
    let str = String::from("Hello");
    str.debug_prints();
    str.display_prints();

    person.debug_prints();
    // person.display_prints(); // Display가 없으므로 Error 발생

    building.debug_prints();
    // building.display_prints(); // Display가 없으므로 Error 발생
}
