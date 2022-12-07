fn find_disjoint_window(s: &[u8], w: usize) -> usize {
    let mut last_known_position = [0; 256];
    let mut start_disjoint = 0;
    for i in 0..s.len() {
        if i >= start_disjoint + w {
            return i;
        }
        start_disjoint = start_disjoint.max(last_known_position[s[i] as usize] + 1);
        last_known_position[s[i] as usize] = i;
    }
    usize::MAX
}

pub(crate) fn run() -> (usize, usize) {
    let input = include_bytes!("data/day06.txt");
    (find_disjoint_window(input, 4), find_disjoint_window(input, 14))
}

#[test]
fn test_solution() {}
