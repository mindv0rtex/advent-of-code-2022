#[repr(C)]
pub struct AlignedAs<Align, Bytes: ?Sized> {
    _align: [Align; 0],
    pub bytes: Bytes,
}

macro_rules! include_bytes_align_as {
    ($align_ty:ty, $path:literal) => {{
        static ALIGNED: &AlignedAs<$align_ty, [u8]> = &AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };
        &ALIGNED.bytes
    }};
}

#[allow(dead_code)]
fn find_magic() {
    let mut r = rand::thread_rng();
    let pairs =
        bytemuck::cast_slice::<_, u32>(include_bytes_align_as!(u32, "data/day02_pairs.txt"));
    'outer: loop {
        let mul: u32 = rand::Rng::gen(&mut r);
        let shift = 27; // 32 - 27 = 5 bits left, 2^5 = 32 for shifts around the u32
        let mut indexes = [0_u8; 9]; // x stores bit offset within a u32
        for (idx, &p) in indexes.iter_mut().zip(pairs) {
            // Each index is between 0 and 32 (only 5 bits long)
            *idx = (p.wrapping_mul(mul) >> shift) as u8;
        }
        let scores_p1 = [4, 8, 3, 1, 5, 9, 7, 2, 6];
        let scores_p2 = [3, 4, 8, 1, 5, 9, 2, 6, 7];
        // Squeeze score tables into a single u32. This may naturally result in collisions.
        let (mut packed_p1, mut packed_p2) = (0u32, 0u32);
        for i in 0..9 {
            packed_p1 |= scores_p1[i] << indexes[i];
            packed_p2 |= scores_p2[i] << indexes[i];
        }

        // Check for collissions.
        for i in 0..9 {
            if scores_p1[i] != (packed_p1 >> indexes[i]) & 0xf
                || scores_p2[i] != (packed_p2 >> indexes[i]) & 0xf
            {
                continue 'outer;
            }
        }
        println!("mul:        {mul:#08x}");
        println!("packed_p1:  {packed_p1:#08x}");
        println!("packed_p2:  {packed_p2:#08x}");
        println!("indexes:    {indexes:?}");
        break;
    }
}

fn find_score(input: &[u8], scores: u32) -> Result<u32, bytemuck::PodCastError> {
    Ok(bytemuck::try_cast_slice::<_, u32>(input)?
        .iter()
        .map(|&x| (scores >> (x.wrapping_mul(0x55c11a41) >> 27)) & 0xf)
        .sum())
}

pub(crate) fn run() -> Result<(u32, u32), bytemuck::PodCastError> {
    let input = include_bytes_align_as!(u32, "data/day02.txt");
    Ok((
        find_score(input, 0xa2623179)?,
        find_score(input, 0xa311bb29)?,
    ))
}

#[test]
fn test_solution() {
    let values = b"A Y\nB X\nC Z\n";
    assert_eq!(find_score(&values[..], 0xa2623179).unwrap(), 15);
    assert_eq!(find_score(&values[..], 0xa311bb29).unwrap(), 12);
}
