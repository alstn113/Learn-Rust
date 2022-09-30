use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    // 바로 바로 칠 수 입력받을 수 있음.
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b)
                } else {
                    println!("{:?} {}\r", b, c)
                }
                if b == to_ctrl_byte('c') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}

// ctrl + key 처리를 위함.
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

// 에러 처리를 위함.
fn die(e: std::io::Error) {
    panic!("{}", e);
}
