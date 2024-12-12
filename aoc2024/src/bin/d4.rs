use anyhow::{Context, Result};

fn check_xmas(canvas: &Vec<Vec<u8>>, i: usize, j: usize, di: isize, dj: isize) -> Option<u32> {
    if "XMAS"
        .bytes()
        .enumerate()
        .map(|(idx, b)| {
            Some(
                *canvas
                    .get(i.checked_add_signed(idx as isize * di)?)?
                    .get(j.checked_add_signed(idx as isize * dj)?)?
                    == b,
            )
        })
        .all(|e| e == Some(true))
    {
        Some(1)
    } else {
        None
    }
}

fn check_x_max(canvas: &Vec<Vec<u8>>, i: usize, j: usize) -> Option<u32> {
    let diagonals = [[(-1, -1), (0, 0), (1, 1)], [(-1, 1), (0, 0), (1, -1)]];
    if diagonals
        .into_iter()
        .map(|d| {
            d.into_iter()
                .map(|(di, dj)| {
                    canvas
                        .get(i.checked_add_signed(di)?)?
                        .get(j.checked_add_signed(dj)?)
                })
                .filter_map(|e| e.copied())
                .collect::<Vec<u8>>()
        })
        // For each of the two diagonals, have to be either "MAS" or its reverse.
        .all(|d| {
            [&d, &d.iter().rev().copied().collect::<Vec<_>>()]
                .iter()
                .any(|e| e == &"SAM".as_bytes())
        })
    {
        Some(1)
    } else {
        None
    }
}

fn main() -> Result<()> {
    let mut count = 0;
    let mut count2 = 0;

    let canvas: Vec<Vec<u8>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().into())
        .collect();

    let h = canvas.len();
    let w = canvas.first().unwrap().len();
    for i in 0..h {
        for j in 0..w {
            for di in -1..=1 {
                for dj in -1..=1 {
                    if let Some(n) = check_xmas(&canvas, i, j, di, dj) {
                        count += n;
                    }
                }
            }
            if let Some(n) = check_x_max(&canvas, i, j) {
                count2 += n;
            }
        }
    }

    println!("{count},{count2}");
    Ok(())
}
