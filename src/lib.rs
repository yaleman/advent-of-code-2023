use std::fmt::Display;

///```
/// let digit: u8 = advent_of_code_2023::Digit::One.into();
/// assert_eq!(digit, 1);
///```
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl From<Digit> for u8 {
    fn from(value: Digit) -> Self {
        value as u8
    }
}

impl From<&Digit> for u8 {
    fn from(value: &Digit) -> Self {
        *value as u8
    }
}

impl From<Digit> for &'static str {
    fn from(value: Digit) -> Self {
        match value {
            Digit::Zero => "zero",
            Digit::One => "one",
            Digit::Two => "two",
            Digit::Three => "three",
            Digit::Four => "four",
            Digit::Five => "five",
            Digit::Six => "six",
            Digit::Seven => "seven",
            Digit::Eight => "eight",
            Digit::Nine => "nine",
            Digit::Ten => "ten",
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: &'static str = (*self).into();
        write!(f, "{}", res)
    }
}

impl Digit {
    pub fn digits() -> Vec<Digit> {
        vec![
            Digit::Zero,
            Digit::One,
            Digit::Two,
            Digit::Three,
            Digit::Four,
            Digit::Five,
            Digit::Six,
            Digit::Seven,
            Digit::Eight,
            Digit::Nine,
            Digit::Ten,
        ]
    }
}
