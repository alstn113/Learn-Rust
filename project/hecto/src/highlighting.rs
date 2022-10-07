use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    Match,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Self::Number => color::Rgb(220, 163, 163),
            Self::Match => color::Rgb(38, 139, 210),
            _ => color::Rgb(225, 225, 226),
        }
    }
}
