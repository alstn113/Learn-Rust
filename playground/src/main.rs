use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_prints<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{}: {} is {}", statement, num_1, num_2);
}

fn main() {
    compare_and_prints("statement", 10, 8)
}
