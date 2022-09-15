use thiserror::Error;

#[derive(Debug)]
struct User {
    points: u32,
    age: u8,
}

impl User {
    fn try_new() -> Result<Self, CompanyError> {
        todo!()
    }
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

fn main() {
    let error = CompanyError::TooBig(1000000);
    print!("{}", error);
}
