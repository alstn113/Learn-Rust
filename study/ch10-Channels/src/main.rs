// mpsc : multiple producer single consumer

use std::any::Any;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn sleepy(time: u64) {
    println!("sleeping for {} seconds", time);
    sleep(Duration::from_millis(time));
}

#[derive(Debug)]
struct Book {
    name: String,
}

#[derive(Debug)]
struct Magazine {
    name: String,
}

// turn to trait object
fn book(name: &str) -> Box<dyn Any + Send> {
    let book = Book {
        name: name.to_string(),
    };
    Box::new(book)
}

fn magazine(name: &str) -> Box<dyn Any + Send> {
    let magazine = Magazine {
        name: name.to_string(),
    };
    Box::new(magazine)
}

fn main() {
    let (sender, recevier) = channel();

    let s_1 = sender.clone();
    let s_2 = sender.clone();

    // take by value
    thread::spawn(move || {
        for _ in 0..5 {
            sleepy(1000);
            s_1.send(book("book")).unwrap();
        }
    });

    thread::spawn(move || {
        for _ in 0..5 {
            sleepy(500);
            s_2.send(magazine("magazine")).unwrap();
        }
    });

    // println!("recevier: {:?}", recevier.recv()); 받을 때까지 blocking
    // println!("recevier: {:?}", recevier.recv_timeout(500)); 500ms 동안 기다린다.
    // println!("recevier: {:?}", recevier.try_recv());  바로 받아오기 없으면 Err
    while let Ok(any_type) = recevier.recv() {
        if let Some(book) = any_type.downcast_ref::<Book>() {
            println!("book: {:?}", book);
        } else if let Some(magazine) = any_type.downcast_ref::<Magazine>() {
            println!("magazine: {:?}", magazine);
        } else {
            panic!("unknown type");
        }
    }
}
