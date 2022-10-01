use anyhow::{anyhow, Error as AnyhowError};
use serde::{Deserialize, Serialize};
// SERialize turn into JSON, YAML etc..
// DEserialize turn from JSON, YAML etc..
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    points: u32,
    age: u8,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserRequest {
    points: u32,
    age: u8,
}

impl User {
    fn from_request(request: UserRequest) -> Result<Self, AnyhowError> {
        if request.age < 120 && request.points < 10000 {
            Ok(Self {
                age: request.age,
                points: request.points,
            })
        } else {
            Err(anyhow!("Too Old or Too Big"))
        }
    }
}

fn main() {
    let request = r#"{
        "age": 60,
        "points": 1000
    }"#;

    let user_request: UserRequest = serde_json::from_str(request).unwrap();
    let user_try = User::from_request(user_request);
    println!("{:?}", user_try);
}
