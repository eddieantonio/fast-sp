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
use std::simd::SimdPartialEq;

pub fn rust_emulate_numpy(s: &CStr) -> isize {
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
