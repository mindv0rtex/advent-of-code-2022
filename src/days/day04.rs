use crate::utils::*;

unsafe fn parse_line(s: &mut &[u8]) -> [i16; 4] {
    [(); 4].map(|_| {
        let x = [get_at(s, 0), get_at(s, 1)];
        if x[1] & 0b10000 != 0 {
            advance(s, 3);
            i16::from_be_bytes(x)
        } else {
            advance(s, 2);
            x[0] as _
        }
    })
}

fn find_solution(mut input: &[u8]) -> (u32, u32) {
    let mut solution = (0, 0);
    while !input.is_empty() {
        let [a, b, c, d] = unsafe { parse_line(&mut input) };
        solution.0 += ((a <= c && b >= d) || (c <= a && d >= b)) as u32;
        solution.1 += (a <= d && b >= c) as u32;
    }
    solution
}

pub(crate) fn run() -> (u32, u32) {
    let input = include_bytes!("data/day04.txt");
    find_solution(input)
}

#[test]
fn test_solution() {
    let input = b"2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    assert_eq!(find_solution(input).0, 2);
    assert_eq!(find_solution(input).1, 4);
}
