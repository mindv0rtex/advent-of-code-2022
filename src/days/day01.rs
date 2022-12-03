use std::num::ParseIntError;

fn find_max_calories<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<(u32, u32), ParseIntError> {
    let mut top_sums: [u32; 4] = [0; 4];
    for line in lines {
        if line.is_empty() {
            top_sums.sort();
            top_sums[0] = 0;
        } else {
            top_sums[0] += line.parse::<u32>()?;
        }
    }
    top_sums.sort();
    Ok((top_sums[3], top_sums[1..].iter().sum()))
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
