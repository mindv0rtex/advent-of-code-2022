use arrayvec::ArrayVec;

use crate::utils::*;

const K: usize = 16;
const N: usize = 64;
type Stack = ArrayVec<u8, N>;

unsafe fn parse_stacks(s: &mut &[u8]) -> ArrayVec<Stack, K> {
    let line_len = memchr(s, b'\n') + 1;
    let n_stacks = (line_len + 1) >> 2;
    let stacks_len = memchr(s, b'1') - 1;
    let max_height = stacks_len / line_len;

    let mut stacks = ArrayVec::new();
    for _ in 0..n_stacks {
        stacks.push(ArrayVec::new());
    }
    let mut row_end = stacks_len;
    for _ in 0..max_height {
        let row = &s[row_end - line_len..row_end];
        for j in 0..n_stacks {
            let c = get_at(row, (j << 2) + 1);
            if c != b' ' {
                stacks[j].push(c);
            }
        }
        row_end -= line_len;
    }

    advance(s, stacks_len + line_len + 1);
    stacks
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

unsafe fn parse_moves(s: &mut &[u8]) -> Vec<Move> {
    let mut moves = Vec::with_capacity(1024);
    while !s.is_empty() {
        advance(s, 5);
        let mut n = get_at(s, 0) - b'0';
        if (b'0'..=b'9').contains(&get_at(s, 1)) {
            n = n * 10 + get_at(s, 1) - b'0';
            advance(s, 1);
        }
        let amount = usize::from(n);
        let from = usize::from(get_at(s, 7) - b'0' - 1);
        let to = usize::from(get_at(s, 12) - b'0' - 1);
        advance(s, 14);
        moves.push(Move { amount, from, to });
    }
    moves
}

struct Pos {
    col: usize,
    offset: usize,
}

fn find_solution(mut input: &[u8], is_9000: bool) -> String {
    let stacks = unsafe { parse_stacks(&mut input) };
    let moves = unsafe { parse_moves(&mut input) };

    let mut positions = ArrayVec::<Pos, K>::new();
    for col in 0..stacks.len() {
        positions.push(Pos { col, offset: 0 });
    }

    for mv in moves.iter().rev() {
        for pos in positions.iter_mut() {
            if pos.col == mv.from {
                pos.offset += mv.amount;
            } else if pos.col == mv.to {
                if pos.offset < mv.amount {
                    if is_9000 {
                        pos.offset = mv.amount - pos.offset - 1;
                    }
                    pos.col = mv.from;
                } else {
                    pos.offset -= mv.amount;
                }
            }
        }
    }
    positions
        .iter()
        .map(|pos| char::from(stacks[pos.col][stacks[pos.col].len() - 1 - pos.offset as usize]))
        .collect()
}

pub(crate) fn run() -> (String, String) {
    let input = include_bytes!("data/day05.txt");
    (find_solution(input, true), find_solution(input, false))
}

#[test]
fn test_solution() {
    let input = b"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(&find_solution(input, |offset, amt| amt - offset - 1), &"CMZ");
    assert_eq!(&find_solution(input, |offset, _| offset), &"MCD");
}
