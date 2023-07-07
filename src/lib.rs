// Copyright (C) 2023  Eddie Antonio Santos
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(portable_simd)]

use std::ffi::CStr;

pub fn count_iter(s: &CStr) -> isize {
    s.to_bytes()
        .iter()
        .map(|c| match c {
            b'p' => 1,
            b'n' => -1,
            _ => 0,
        })
        .sum()
}

pub fn count_for_loop(s: &CStr) -> isize {
    let mut result = 0;
    for &c in s.to_bytes() {
        if c == b'p' {
            result += 1;
        } else if c == b'n' {
            result -= 1;
        }
    }

    result
}

pub fn count_simd(s: &CStr) -> isize {
    use std::simd::{u8x16, SimdInt, SimdPartialEq};
    let bytes = s.to_bytes();
    let (prefix, middle, suffix) = bytes.as_simd();

    let p = u8x16::splat(b'p');
    let n = u8x16::splat(b'n');

    let mut result = 0;
    for &window in middle {
        let ps = window.simd_eq(p);
        let ns = window.simd_eq(n);
        let neg_ps = ps.to_int();
        let neg_ns = ns.to_int();
        let pairwise = neg_ns.saturating_add(-neg_ps);

        result += pairwise.reduce_sum() as isize;
    }

    _count_scalar(prefix) + result + _count_scalar(suffix)
}

pub fn _count_scalar(s: &[u8]) -> isize {
    let mut result = 0;
    for &c in s {
        if c == b'p' {
            result += 1;
        } else if c == b'n' {
            result -= 1;
        }
    }

    result
}

mod internal {
    #[link(name = "count")]
    extern "C" {
        pub(crate) fn count_c(s: *const i8) -> isize;
    }
}

#[inline(always)]
pub fn count_c(s: &CStr) -> isize {
    unsafe { internal::count_c(s.as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    const SMALL_SENTENCE: &str = "ppnpnpp";
    const SMALL_SENTENCE_ANSWER: isize = 3;
    const BIG_SENTENCE: &str = "pppppnpppnpppnn.npppnpnnnpnnnnpn";
    const BIG_SENTENCE_ANSWER: isize = 3;

    #[test]
    fn it_works_iter() {
        let sentence = CString::new(SMALL_SENTENCE).unwrap();
        assert_eq!(SMALL_SENTENCE_ANSWER, count_iter(sentence.as_c_str()));
    }

    #[test]
    fn it_works_iter_big() {
        let sentence = CString::new(BIG_SENTENCE).unwrap();
        assert_eq!(BIG_SENTENCE_ANSWER, count_iter(sentence.as_c_str()));
    }

    #[test]
    fn it_works_for_loop() {
        let sentence = CString::new("ppnpnpp").unwrap();
        assert_eq!(SMALL_SENTENCE_ANSWER, count_for_loop(sentence.as_c_str()));
    }

    #[test]
    fn it_works_for_loop_big() {
        let sentence = CString::new(BIG_SENTENCE).unwrap();
        assert_eq!(BIG_SENTENCE_ANSWER, count_for_loop(sentence.as_c_str()));
    }

    #[test]
    fn it_works_c() {
        let sentence = CString::new("ppnpnpp").unwrap();
        assert_eq!(SMALL_SENTENCE_ANSWER, count_c(sentence.as_c_str()));
    }

    #[test]
    fn it_works_c_big() {
        let sentence = CString::new(BIG_SENTENCE).unwrap();
        assert_eq!(BIG_SENTENCE_ANSWER, count_c(sentence.as_c_str()));
    }

    #[test]
    fn it_works_simd() {
        let sentence = CString::new("ppnpnpp").unwrap();
        // will not actually use SIMD:
        assert_eq!(SMALL_SENTENCE_ANSWER, count_simd(sentence.as_c_str()));
    }

    #[test]
    fn it_works_simd_big() {
        let sentence = CString::new(BIG_SENTENCE).unwrap();
        assert_eq!(BIG_SENTENCE_ANSWER, count_simd(sentence.as_c_str()));
    }
}
