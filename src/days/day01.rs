use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

fn find_max_calories<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<(u32, u32), ParseIntError> {
    let groups = lines.map(|l| l.trim()).group_by(|l| !l.is_empty());
    let mut top_three = [0; 3];
    for (has_items, group) in groups.into_iter() {
        if !has_items {
            continue;
        }
        let sum = group.map(u32::from_str).sum::<Result<_, _>>()?;
        for i in 0..3 {
            if sum < top_three[i] {
                continue;
            }
            top_three.copy_within(i..2, i + 1);
            top_three[i] = sum;
            break;
        }
    }
    Ok((top_three[0], top_three.iter().sum()))
}

pub(crate) fn run() -> Result<(u32, u32), ParseIntError> {
    let input = include_str!("data/day01.txt");
    let mut lines = input.lines();
    find_max_calories(&mut lines)
}

#[test]
fn test_solution() {
    let values = [
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];
    assert_eq!(
        find_max_calories(values.into_iter()).unwrap(),
        (24000, 45000)
    );
}
