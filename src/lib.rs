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

    #[test]
    fn it_works_iter() {
        let sentence = CString::new("ppnpnpp").unwrap();
        assert_eq!(3, count_iter(sentence.as_c_str()));
    }

    #[test]
    fn it_works_for_loop() {
        let sentence = CString::new("ppnpnpp").unwrap();
        assert_eq!(3, count_for_loop(sentence.as_c_str()));
    }

    #[test]
    fn it_works_c() {
        let sentence = CString::new("ppnpnpp").unwrap();
        assert_eq!(3, count_c(sentence.as_c_str()));
    }
}
