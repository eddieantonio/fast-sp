use std::ffi::CStr;
use std::simd::{u8x16, SimdInt, SimdPartialEq};

/// Counts using Rust iterators.
pub fn count_iter(s: &CStr) -> isize {
    s.to_bytes()
        .iter()
        .map(|c| match c {
            b's' => 1,
            b'p' => -1,
            _ => 0,
        })
        .sum()
}

/// Counts using an explicit for-loop with mutable state.
pub fn count_for_loop(s: &CStr) -> isize {
    let mut result = 0;
    for &c in s.to_bytes() {
        if c == b's' {
            result += 1;
        } else if c == b'p' {
            result -= 1;
        }
    }

    result
}

/// Counts using portable_simd.
pub fn count_simd(s: &CStr) -> isize {
    let bytes = s.to_bytes();
    let (prefix, middle, suffix) = bytes.as_simd();

    let s = u8x16::splat(b's');
    let p = u8x16::splat(b'p');

    let mut result = 0;
    for &window in middle {
        let ss = window.simd_eq(s);
        let ps = window.simd_eq(p);
        let neg_ss = ss.to_int();
        let neg_ps = ps.to_int();
        let pairwise = neg_ps - neg_ss;

        result += pairwise.reduce_sum() as isize;
    }

    _count_scalar(prefix) + result + _count_scalar(suffix)
}

// Basically the for-loop version, but takes a slice.
#[inline(always)]
fn _count_scalar(s: &[u8]) -> isize {
    let mut result = 0;
    for &c in s {
        if c == b's' {
            result += 1;
        } else if c == b'p' {
            result -= 1;
        }
    }

    result
}

/// Count implementation written in C. See src/count.c
#[inline(always)]
pub fn count_c(s: &CStr) -> isize {
    // Tiny wrapper that changes Rust's borrowed CStr and converts it into C's const char*.
    use std::ffi::c_char;

    // This will link to libcount.a and use its count_c function.
    // The scope of this external symbol is entirely internal to this function.
    #[link(name = "count", kind = "static")]
    extern "C" {
        fn count_c(s: *const c_char) -> isize;
    }

    unsafe { count_c(s.as_ptr()) }
}