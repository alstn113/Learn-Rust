fn loop() {
    let mut counter = 0;
    loop {
        println!("looping");
        counter += 1;
        if counter == 10 {
            break;
        }
    }
}

fn while() {
    let mut counter = 0;
    while counter < 10 {
        println!("looping");
        counter += 1;
    }
}

fn for() {
    for i in 0..10 {
        println!("looping");
    }
}

fn main() {
    loop();
    while();
    for();
}
