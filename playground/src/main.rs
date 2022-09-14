fn main() {
    let first = vec![None, Some("Success"), None];
    let second = vec![Some("Success"), Some("Success"), None];
    let third = vec![None, Some("Success"), None];

    for i in 0..first.len() {
        println!("{:?}", first[i].or(second[i]).or(third[i]));
    }
}
