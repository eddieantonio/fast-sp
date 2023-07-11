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

/// Count implementation written in C. See src/count.c
#[inline(always)]
pub fn c_while_loop(s: &CStr) -> isize {
    // Tiny wrapper that changes Rust's borrowed CStr and converts it into C's const char*.
    use std::ffi::c_char;

    // This will link to libcount.a and use its count_c function.
    // The scope of this external symbol is entirely internal to this function.
    #[link(name = "count", kind = "static")]
    extern "C" {
        fn while_not_zero(s: *const c_char) -> isize;
    }

    unsafe { while_not_zero(s.as_ptr()) }
}

/// Owen's original implementation written in C. See src/count.c
#[inline(always)]
pub fn c_original(s: &CStr) -> isize {
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
pub fn c_for_loop(s: &CStr) -> isize {
    // Tiny wrapper that changes Rust's borrowed CStr and converts it into C's const char*.
    use std::ffi::c_char;

    // This will link to libcount.a and use its count_c function.
    // The scope of this external symbol is entirely internal to this function.
    #[link(name = "count", kind = "static")]
    extern "C" {
        fn with_explicit_size(s: *const c_char, n: usize) -> i32;
    }

    let s = s.to_bytes();
    unsafe { with_explicit_size(s.as_ptr() as *const c_char, s.len()) as isize }
}
