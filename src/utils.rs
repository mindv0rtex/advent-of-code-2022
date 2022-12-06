use core::hint::unreachable_unchecked;

#[inline]
pub unsafe fn advance(s: &mut &[u8], n: usize) {
    *s = std::slice::from_raw_parts(s.as_ptr().add(n), s.len().saturating_sub(n));
}

#[inline]
pub unsafe fn get_at(s: &[u8], i: usize) -> u8 {
    *s.get_unchecked(i)
}

#[inline]
/// Returns the index corresponding to the first occurrence of a byte in a byte slice.
pub fn memchr(s: &[u8], c: u8) -> usize {
    memchr::memchr(c, s).unwrap_or_else(|| unsafe { unreachable_unchecked() })
}
