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

use std::ffi::{c_char, CStr};

macro_rules! define_ffi {
    ($(#[$meta:meta])* $name: ident => $link_name: ident *const c_char) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(s: &CStr) -> isize {
            // Tiny wrapper that changes Rust's borrowed CStr and converts it into C's const char*.
            // This will link to libcount.a and use its count_c function.
            // The scope of this external symbol is entirely internal to this function.
            #[link(name = "count", kind = "static")]
            extern "C" {
                fn $link_name(s: *const c_char) -> isize;
            }

            unsafe { $link_name(s.as_ptr()) }
        }
    };

    ($(#[$meta:meta])* $name: ident => $link_name: ident usize) => {
        $(#[$meta])*
        #[inline(always)]
        pub fn $name(s: &CStr) -> isize {
            // The scope of this external symbol is entirely internal to this function.
            #[link(name = "count", kind = "static")]
            extern "C" {
                fn $link_name(s: *const c_char, n: usize) -> i32;
            }

            let s = s.to_bytes();
            unsafe { $link_name(s.as_ptr() as *const c_char, s.len()) as isize }
        }
    };
}

define_ffi!(
    /// Owen's original implementation written in C. See c/original.c
    c_original => run_switches *const c_char
);

define_ffi!(
    /// Count implementation written in C. See c/while-not-zero.c
    c_while_loop => while_not_zero *const c_char
);

define_ffi!(
    /// Owen's implementation, with explicit size (does not check for null terminator).
    /// See c/with-explicit-size.c
    c_for_loop => with_explicit_size usize
);

define_ffi!(
    /// Using a state machine approach from <https://github.com/robertdavidgraham/wc2>
    /// See c/state-machine.c
    c_state_machine => c_state_machine usize
);

define_ffi!(
    /// Like the [c_state_machine], but without the state machine.
    /// See c/count-machine.c
    c_count_machine => c_count_machine usize
);
