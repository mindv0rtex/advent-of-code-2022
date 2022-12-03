use std::num::ParseIntError;

fn find_max_calories<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<(u32, u32), ParseIntError> {
    let mut arr: [u32; 4] = [0; 4];
    for v in lines {
        if v.is_empty() {
            arr.sort();
            arr[0] = 0;
        } else {
            arr[0] += v.parse::<u32>()?;
        }
    }
    Ok((arr[3], arr[1] + arr[2] + arr[3]))
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
