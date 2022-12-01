use anyhow::{Context, Result};
use itertools::Itertools;

fn find_max_calories<'a>(lines: impl Iterator<Item = &'a str>) -> Result<(u32, u32)> {
    let groups = lines.map(|l| l.trim()).group_by(|l| !l.is_empty());
    let mut sums: Vec<u32> = groups
        .into_iter()
        .filter_map(|(non_empty, g)| non_empty.then_some(g))
        .map(|g| g.map(|l| l.parse::<u32>()).fold_ok(0, std::ops::Add::add))
        .try_collect()
        .with_context(|| "Parsing input as ints failed somewhere")?;

    sums.select_nth_unstable_by_key(2, |s| std::cmp::Reverse(*s));
    let max_calories = sums[..3].iter().copied().max().unwrap_or(0);
    let top_3_sum = sums[..3].iter().sum();
    Ok((max_calories, top_3_sum))
}

pub(crate) fn run() -> Result<(u32, u32)> {
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
