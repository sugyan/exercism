use std::fmt::{Display, Formatter, Result};

struct Bottles {
    n: u32,
    capitalize: bool,
}

impl Bottles {
    fn new(n: u32, capitalize: bool) -> Self {
        Self { n, capitalize }
    }
}

impl Display for Bottles {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.n {
            0 if self.capitalize => write!(f, "No more bottles"),
            0 => write!(f, "no more bottles"),
            1 => write!(f, "1 bottle"),
            n => write!(f, "{} bottles", n),
        }
    }
}

pub fn verse(n: u32) -> String {
    let mut ret = format!(
        "{} of beer on the wall, {} of beer.\n",
        Bottles::new(n, true),
        Bottles::new(n, false)
    );
    match n {
        0 => ret += "Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        1 => ret += "Take it down and pass it around, no more bottles of beer on the wall.\n",
        n => {
            ret += &format!(
                "Take one down and pass it around, {} of beer on the wall.\n",
                Bottles::new(n - 1, false)
            )
        }
    };
    ret
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
