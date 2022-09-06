fn fn_loop() {
    let mut counter = 0;
    loop {
        println!("fn_loop: {}", counter);
        counter += 1;
        if counter == 5 {
            break;
        }
    }
}

fn fn_loop_name() {
    let mut counter = 0;
    'outer: loop {
        println!("fn_loop_name: {}", counter);
        counter += 1;
        if counter == 5 {
            break 'outer;
        }
    }
}

fn fn_while() {
    let mut counter = 0;
    while counter < 5 {
        println!("fn_while: {}", counter);
        counter += 1;
    }
}

fn fn_for() {
    for i in 0..5 {
        println!("fn_for: {}", i);
    }
}

fn main() {
    fn_loop();
    fn_loop_name();
    fn_while();
    fn_for();
}
