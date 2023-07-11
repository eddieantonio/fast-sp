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

use std::ffi::CStr;
use std::simd::{u8x16, SimdInt, SimdPartialEq};

/// Counts using Rust iterators.
///
/// See assembly in Compiler Explorer: <https://godbolt.org/z/jja8PMqTr>
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
///
/// See assembly in Compiler Explorer: <https://godbolt.org/z/P1jGzroqY>
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
///
/// See assembly in Compiler Explorer: <https://godbolt.org/z/vzjbojYna>
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

pub fn emulate_numpy(s: &CStr) -> isize {
    let bytes = s.to_bytes();

    let ps = vec_eq(bytes, b'p');
    let num_ps = nonzeros(&ps) as isize;

    let ss = vec_eq(bytes, b's');
    let num_ss = nonzeros(&ss) as isize;

    num_ss - num_ps
}

#[inline(never)]
pub fn vec_eq(s: &[u8], value: u8) -> Vec<bool> {
    s.iter().map(|&c| c == value).collect()
}

#[inline(never)]
pub fn vec_eq_simd(input: &[u8], value: u8) -> Vec<bool> {
    const N: usize = 32;

    let mut buffer = Vec::<bool>::with_capacity(input.len());

    let n_initial_bytes = input.len() % N;
    for &byte in &input[..n_initial_bytes] {
        buffer.push(byte == value);
    }

    unsafe {
        // Pretend the buffer is large enough. UB be here:
        buffer.set_len(input.len());
    }

    _vec_eq_fast::<N>(
        &input[n_initial_bytes..],
        &mut buffer[n_initial_bytes..],
        value,
    );

    buffer
}

pub fn vec_eq_only_simd(input: &[u8], buffer: &mut [bool], value: u8) {
    _vec_eq_fast::<32>(input, buffer, value)
}

#[inline]
fn _vec_eq_fast<const N: usize>(input: &[u8], buffer: &mut [bool], value: u8)
where
    std::simd::LaneCount<N>: std::simd::SupportedLaneCount,
{
    use std::simd::Simd;

    assert_eq!(input.len() % N, 0);
    assert_eq!(input.len(), buffer.len());

    let one = Simd::<i8, N>::splat(1);
    let value = Simd::<u8, N>::splat(value);
    let buffer = unsafe { std::mem::transmute::<&mut [bool], &mut [i8]>(buffer) };
    for (output_chunk, input_chunk) in buffer.chunks_exact_mut(N).zip(input.chunks_exact(N)) {
        let input_chunk = Simd::<u8, N>::from_slice(input_chunk);
        let result = input_chunk.simd_eq(value).to_int() & one;
        result.copy_to_slice(output_chunk);
    }
}

#[allow(clippy::uninit_vec)]
pub fn vec_eq_do_nothing_but_allocate(input: &[u8], _value: u8) -> Vec<bool> {
    let mut buffer = Vec::<bool>::with_capacity(input.len());

    unsafe {
        // Pretend the buffer is large enough. UB be here:
        buffer.set_len(input.len());
    }
    buffer
}

pub fn vec_eq_only_prefix(input: &[u8], value: u8) -> Vec<bool> {
    const N: usize = 32;
    let mut buffer = Vec::<bool>::with_capacity(input.len());
    let n_initial_bytes = input.len() % N;
    for &byte in &input[..n_initial_bytes] {
        buffer.push(byte == value);
    }

    buffer
}

#[inline(never)]
pub fn nonzeros(s: &[bool]) -> usize {
    s.iter().map(|&b| b as usize).sum()
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

/// Owen's original implementation written in C. See src/count.c
#[inline(always)]
pub fn count_c_owen(s: &CStr) -> isize {
    // Tiny wrapper that changes Rust's borrowed CStr and converts it into C's const char*.
    use std::ffi::c_char;

    // This will link to libcount.a and use its count_c function.
    // The scope of this external symbol is entirely internal to this function.
    #[link(name = "count", kind = "static")]
    extern "C" {
        fn run_switches(s: *const c_char) -> i32;
    }

    unsafe { run_switches(s.as_ptr()) as isize }
}

/// Owen's implementation, with explicit size (does not check for null terminator).
#[inline(always)]
pub fn count_c_owen_sized(s: &CStr) -> isize {
    // Tiny wrapper that changes Rust's borrowed CStr and converts it into C's const char*.
    use std::ffi::c_char;

    // This will link to libcount.a and use its count_c function.
    // The scope of this external symbol is entirely internal to this function.
    #[link(name = "count", kind = "static")]
    extern "C" {
        fn count_c_owen_sized(s: *const c_char, n: usize) -> i32;
    }

    let s = s.to_bytes();
    unsafe { count_c_owen_sized(s.as_ptr() as *const c_char, s.len()) as isize }
}
