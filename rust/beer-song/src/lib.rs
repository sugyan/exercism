use std::fmt::Write as FmtWrite;

fn bottle_numeral(n: u32, capitalize: bool) -> String {
    match n {
        0 if capitalize => String::from("No more bottles"),
        0 => String::from("no more bottles"),
        1 => String::from("1 bottle"),
        n => format!("{} bottles", n),
    }
}

pub fn verse(n: u32) -> String {
    let mut ret = String::new();
    writeln!(
        &mut ret,
        "{} of beer on the wall, {} of beer.",
        bottle_numeral(n, true),
        bottle_numeral(n, false)
    )
    .ok();
    match n {
        0 => writeln!(
            &mut ret,
            "Go to the store and buy some more, 99 bottles of beer on the wall."
        ),
        1 => writeln!(
            &mut ret,
            "Take it down and pass it around, no more bottles of beer on the wall."
        ),
        n => writeln!(
            &mut ret,
            "Take one down and pass it around, {} of beer on the wall.",
            bottle_numeral(n - 1, false)
        ),
    }
    .ok();
    ret
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ret = Vec::new();
    (end..=start).rev().for_each(|n| ret.push(verse(n)));
    ret.join("\n")
}
