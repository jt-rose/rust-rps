use std::fmt;

// declare game enums
pub enum RspChoice {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for RspChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RspChoice::Rock => write!(f, "Rock"),
            RspChoice::Paper => write!(f, "Paper"),
            RspChoice::Scissors => write!(f, "Scissors"),
        }
    }
}
