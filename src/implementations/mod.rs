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

mod c_ffi;
mod emulate_numpy;
mod rust_for_loop;
mod rust_iter;
mod rust_portable_simd;

pub use c_ffi::*;
pub use emulate_numpy::*;
pub use rust_for_loop::rust_for_loop;
pub use rust_iter::rust_iter;
pub use rust_portable_simd::rust_portable_simd;
