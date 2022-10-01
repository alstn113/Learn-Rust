// Iterator = a collection of things that you can call .next() on

// .iter() - iterator of references &T
// .iter_mut() - iterator of mutable references &mut T
// .iter_into() - consuming iterator

fn main() {
    let vector1 = vec![1, 2, 3];

    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    println!("vector1_1: {vector1_a:?}"); // vector1_1: [2, 3, 4]

    let vector1_b: Vec<i32> = vector1.into_iter().map(|x| x * 10).collect();
    println!("vector1_1: {vector1_b:?}",); // vector1_1: [10, 20, 30]

    let mut vector2 = vec![1, 2, 3];
    vector2.iter_mut().for_each(|x| *x += 100);
    println!("vector2: {vector2:?}"); // vector2: [101, 102, 103]
}
