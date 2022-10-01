// 라이브러리는 그냥 Cargo.toml에 추가되는 듯?

use anyhow::{anyhow, Context, Error};

fn try_to_make_numbers(int: &str, float: &str) -> Result<(i32, f64), Error> {
    let my_integer = int
        .parse::<i32>()
        .with_context(|| "Extra integer info is here")?;
    let my_float = float
        .parse::<f64>()
        .with_context(|| "Extra float info is here")?;

    if my_integer == 9 {
        return Err(anyhow!("x should not be 9"));
    }

    Ok((my_integer, my_float))
}

fn main() {
    let first_try = try_to_make_numbers("8", "dqwd");
    let second_try = try_to_make_numbers("asdfadsf", "3.14");
    let third_try = try_to_make_numbers("9", "3.14");
    println!("{:?}", first_try);
    println!("{:?}", second_try);
    println!("{:?}", third_try);
}
