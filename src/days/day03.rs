use itertools::Itertools;

fn pack_to_bitset(compartment: &[u8]) -> u64 {
    compartment
        .iter()
        .fold(0, |acc, x| acc | (1u64 << (x - b'A')))
}

fn priority(x: u32) -> u32 {
    if x < 26 {
        x + 27
    } else {
        x - 31
    }
}

fn find_solution(input: &[u8]) -> (u32, u32) {
    input
        .split(|&b| b == b'\n')
        .filter(|rucksack| !rucksack.is_empty())
        .chunks(3)
        .into_iter()
        .map(|rucksacks| {
            let mut group = !0;
            let mut sum = (0, 0);
            for r in rucksacks {
                let (c1, c2) = r.split_at(r.len() / 2);
                let (bitset1, bitset2) = (pack_to_bitset(c1), pack_to_bitset(c2));
                sum.0 += priority((bitset1 & bitset2).trailing_zeros());
                group &= bitset1 | bitset2;
            }
            sum.1 = priority(group.trailing_zeros());
            sum
        })
        .fold((0, 0), |acc, (s1, s2)| (acc.0 + s1, acc.1 + s2))
}

pub(crate) fn run() -> (u32, u32) {
    let input = include_bytes!("data/day03.txt");
    find_solution(input)
}

#[test]
fn test_solution() {
    let input = b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(find_solution(input).0, 157);
    assert_eq!(find_solution(input).1, 70);
}
