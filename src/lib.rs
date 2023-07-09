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

pub mod data;
pub mod implementations;

pub use implementations::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    const SMALL_SENTENCE: &str = "sspspss";
    const SMALL_SENTENCE_ANSWER: isize = 3;
    const BIG_SENTENCE: &str = "ssssspssspssspp.pssspspppsppppsp";
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
        let sentence = CString::new(SMALL_SENTENCE).unwrap();
        assert_eq!(SMALL_SENTENCE_ANSWER, count_for_loop(sentence.as_c_str()));
    }

    #[test]
    fn it_works_for_loop_big() {
        let sentence = CString::new(BIG_SENTENCE).unwrap();
        assert_eq!(BIG_SENTENCE_ANSWER, count_for_loop(sentence.as_c_str()));
    }

    #[test]
    fn it_works_c() {
        let sentence = CString::new(SMALL_SENTENCE).unwrap();
        assert_eq!(SMALL_SENTENCE_ANSWER, count_c(sentence.as_c_str()));
    }

    #[test]
    fn it_works_c_big() {
        let sentence = CString::new(BIG_SENTENCE).unwrap();
        assert_eq!(BIG_SENTENCE_ANSWER, count_c(sentence.as_c_str()));
    }

    #[test]
    fn it_works_simd() {
        let sentence = CString::new(SMALL_SENTENCE).unwrap();
        // will not actually use SIMD:
        assert_eq!(SMALL_SENTENCE_ANSWER, count_simd(sentence.as_c_str()));
    }

    #[test]
    fn it_works_simd_big() {
        let sentence = CString::new(BIG_SENTENCE).unwrap();
        assert_eq!(BIG_SENTENCE_ANSWER, count_simd(sentence.as_c_str()));
    }

    #[test]
    fn test_iter_and_simd_have_identical_results() {
        let sentence = CString::new(data::RANDOM_SP).unwrap();
        let sentence = sentence.as_c_str();
        let len = count_iter(sentence);
        assert_eq!(len, count_simd(sentence));

        let sentence = CString::new(data::RANDOM_PRINTABLE).unwrap();
        let sentence = sentence.as_c_str();
        let len = count_iter(sentence);
        assert_eq!(len, count_simd(sentence));
    }
}
