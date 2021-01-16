const ADJACENT: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let minefield = minefield
        .iter()
        .map(|&row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &c)| {
                    if c == '*' {
                        c
                    } else {
                        let count = ADJACENT
                            .iter()
                            .filter(|(di, dj)| {
                                let i = i as i32 + *di;
                                let j = j as i32 + *dj;
                                (0..minefield.len() as i32).contains(&i)
                                    && (0..row.len() as i32).contains(&j)
                                    && minefield[i as usize][j as usize] == '*'
                            })
                            .count() as u8;
                        if count == 0 {
                            ' '
                        } else {
                            (b'0' + count) as char
                        }
                    }
                })
                .collect()
        })
        .collect()
}
