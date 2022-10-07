use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Self::Number => color::Rgb(220, 163, 163),
            _ => color::Rgb(225, 225, 226),
        }
    }
}
