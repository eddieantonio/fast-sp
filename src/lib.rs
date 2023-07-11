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
#![feature(test)]

extern crate test;

pub mod data;
pub mod implementations;

pub use implementations::*;

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CString;

    macro_rules! test_implementation {
        ($implementation: ident) => {
            mod $implementation {
                use std::ffi::CString;

                #[test]
                fn test_small_sentence() {
                    let sentence = CString::new("sspspss").unwrap();
                    assert_eq!(
                        3,
                        crate::implementations::$implementation(sentence.as_c_str())
                    );
                }

                #[test]
                fn test_big_sentence() {
                    let sentence = CString::new("ssssspssspssspp.pssspspppsppppsp").unwrap();
                    assert_eq!(
                        3,
                        crate::implementations::$implementation(sentence.as_c_str())
                    );
                }
            }
        };
    }

    test_implementation!(c_original);
    test_implementation!(c_for_loop);
    test_implementation!(c_while_loop);
    test_implementation!(rust_emulate_numpy);
    test_implementation!(rust_for_loop);
    test_implementation!(rust_iter);
    test_implementation!(rust_portable_simd);

    #[test]
    fn test_implementations_have_identical_results_only_sp() {
        let buffer = CString::new(data::RANDOM_SP).unwrap();
        let sentence = buffer.as_c_str();
        let count_from_iter = rust_for_loop(sentence);

        assert_eq!(count_from_iter, rust_iter(sentence));
        assert_eq!(count_from_iter, rust_portable_simd(sentence));
        assert_eq!(count_from_iter, c_while_loop(sentence));
        assert_eq!(count_from_iter, rust_emulate_numpy(sentence));
    }

    #[test]
    fn test_implementations_have_identical_results_any_printable() {
        let sentence = CString::new(data::RANDOM_PRINTABLE).unwrap();
        let sentence = sentence.as_c_str();
        let count_from_iter = rust_for_loop(sentence);

        assert_eq!(count_from_iter, rust_iter(sentence));
        assert_eq!(count_from_iter, rust_portable_simd(sentence));
        assert_eq!(count_from_iter, c_while_loop(sentence));
        assert_eq!(count_from_iter, rust_emulate_numpy(sentence));
    }
}

#[cfg(test)]
mod benches {
    macro_rules! bench_implementation {
        ($implementation: ident) => {
            mod $implementation {

                use std::ffi::CString;
                use test::Bencher;

                #[bench]
                fn bench_random_sp(b: &mut Bencher) {
                    let buffer = CString::new(crate::data::RANDOM_SP).unwrap();
                    let sentence = test::black_box(buffer.as_c_str());

                    b.iter(|| crate::implementations::$implementation(sentence));
                }

                #[bench]
                fn bench_random_printable(b: &mut Bencher) {
                    let buffer = CString::new(crate::data::RANDOM_PRINTABLE).unwrap();
                    let sentence = test::black_box(buffer.as_c_str());

                    b.iter(|| crate::implementations::$implementation(sentence));
                }
            }
        };
    }

    macro_rules! bench_vec_eq_implementation {
        ($implementation: ident) => {
            mod $implementation {

                use test::Bencher;

                #[bench]
                fn bench_random_sp(b: &mut Bencher) {
                    let sentence = test::black_box(crate::data::RANDOM_SP.as_bytes());
                    b.iter(|| crate::implementations::$implementation(sentence, b's'));
                }

                #[bench]
                fn bench_random_printable(b: &mut Bencher) {
                    let sentence = test::black_box(crate::data::RANDOM_PRINTABLE.as_bytes());
                    b.iter(|| crate::implementations::$implementation(sentence, b's'));
                }
            }
        };
    }

    bench_implementation!(c_original);
    bench_implementation!(c_for_loop);
    bench_implementation!(c_while_loop);
    bench_implementation!(rust_emulate_numpy);
    bench_implementation!(rust_for_loop);
    bench_implementation!(rust_iter);
    bench_implementation!(rust_portable_simd);

    bench_vec_eq_implementation!(vec_eq);
    bench_vec_eq_implementation!(vec_eq_simd);
    bench_vec_eq_implementation!(vec_eq_do_nothing_but_allocate);
    bench_vec_eq_implementation!(vec_eq_only_prefix);

    mod vec_eq_only_simd {
        use crate::implementations::vec_eq_only_simd;
        use test::Bencher;

        #[bench]
        fn bench_random_sp(b: &mut Bencher) {
            let input = crate::data::RANDOM_SP.as_bytes();
            let value = b's';

            const N: usize = 32;
            let mut buffer = Vec::<bool>::with_capacity(input.len());

            let n_initial_bytes = input.len() % N;
            unsafe {
                // Pretend the buffer is large enough. UB be here:
                buffer.set_len(input.len());
            }

            b.iter(|| {
                vec_eq_only_simd(
                    test::black_box(&input[n_initial_bytes..]),
                    &mut buffer[n_initial_bytes..],
                    value,
                )
            });
        }
    }

    mod nonzero {
        use crate::implementations::{nonzeros, vec_eq};
        use test::Bencher;

        #[bench]
        fn bench_random_sp(b: &mut Bencher) {
            let vec = vec_eq(crate::data::RANDOM_SP.as_bytes(), b's');
            let slice = test::black_box(&vec);
            b.iter(|| nonzeros(slice));
        }

        #[bench]
        fn bench_random_printable(b: &mut Bencher) {
            let vec = vec_eq(crate::data::RANDOM_PRINTABLE.as_bytes(), b's');
            let slice = test::black_box(&vec);
            b.iter(|| nonzeros(slice));
        }
    }
}
