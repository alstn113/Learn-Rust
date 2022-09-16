use thiserror::Error;

#[derive(Debug)]
struct User {
    points: u32,
    age: u8,
}

#[derive(Error, Debug)]
enum CompanyError {
    #[error("Not Enough Data")]
    NotEnoughData,
    #[error("Too Old {0} Can't be over 120")]
    TooOld(u8),
    #[error("Too Big {0} Should be under 10,000 points")]
    TooBig(u32),
    #[error("Must be under 120 and 10,000 points, got {0:?} instead")]
    TooOldAndTooBig(User),
}
impl User {
    fn try_new(age: u8, points: u32) -> Result<Self, CompanyError> {
        use CompanyError::*;
        match (age, points) {
            (age, points) if age > 120 && points > 10000 => {
                Err(TooOldAndTooBig(User { age, points }))
            }
            (_, points) if points > 10000 => Err(TooBig(points)),
            (age, _) if age > 120 => Err(TooOld(age)),
            _ => Ok(Self { age, points }),
        }
    }
}

fn main() {
    let user_request = vec![
        User::try_new(121, 10000),
        User::try_new(60, 1000),
        User::try_new(70, 20000),
        User::try_new(130, 1000),
        User::try_new(50, 500),
    ];

    let users = user_request
        .into_iter()
        .filter_map(|user_request| match user_request {
            Ok(user) => Some(user),
            Err(message) => {
                println!("{message}");
                None
            }
        })
        .collect::<Vec<User>>();
    println!("{users:?}")
}
