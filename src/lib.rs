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

    test_implementation!(count_iter);
    test_implementation!(count_for_loop);
    test_implementation!(count_c);
    test_implementation!(count_simd);
    test_implementation!(count_c_owen);
    test_implementation!(count_c_owen_sized);
    test_implementation!(emulate_numpy);

    #[test]
    fn test_implementations_have_identical_results_only_sp() {
        let buffer = CString::new(data::RANDOM_SP).unwrap();
        let sentence = buffer.as_c_str();
        let count_from_iter = count_for_loop(sentence);

        assert_eq!(count_from_iter, count_iter(sentence));
        assert_eq!(count_from_iter, count_c(sentence));
        assert_eq!(count_from_iter, count_simd(sentence));
        assert_eq!(count_from_iter, emulate_numpy(sentence));
    }

    #[test]
    fn test_implementations_have_identical_results_any_printable() {
        let sentence = CString::new(data::RANDOM_PRINTABLE).unwrap();
        let sentence = sentence.as_c_str();
        let count_from_iter = count_for_loop(sentence);

        assert_eq!(count_from_iter, count_iter(sentence));
        assert_eq!(count_from_iter, count_c(sentence));
        assert_eq!(count_from_iter, count_simd(sentence));
        assert_eq!(count_from_iter, emulate_numpy(sentence));
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

    //bench_implementation!(count_iter);
    //bench_implementation!(count_for_loop);
    //bench_implementation!(count_c);
    bench_implementation!(count_simd);
    //bench_implementation!(count_c_owen);
    //bench_implementation!(count_c_owen_sized);
    bench_implementation!(emulate_numpy);
}
