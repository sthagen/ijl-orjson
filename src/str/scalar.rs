// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::str::pyunicode_new::{
    pyunicode_ascii, pyunicode_fourbyte, pyunicode_onebyte, pyunicode_twobyte,
};

#[inline(never)]
pub fn str_impl_kind_scalar(buf: &str) -> *mut pyo3_ffi::PyObject {
    let num_chars = bytecount::num_chars(buf.as_bytes());
    if buf.len() == num_chars {
        return pyunicode_ascii(buf.as_ptr(), num_chars);
    }
    unsafe {
        let len = buf.len();
        assume!(len > 0);

        if unlikely!(*(buf.as_bytes().as_ptr()) > 239) {
            return pyunicode_fourbyte(buf, num_chars);
        }

        let sptr = buf.as_bytes().as_ptr();

        let mut is_four = false;
        let mut not_latin = false;
        for i in 0..len {
            is_four |= *sptr.add(i) > 239;
            not_latin |= *sptr.add(i) > 195;
        }
        if is_four {
            pyunicode_fourbyte(buf, num_chars)
        } else if not_latin {
            pyunicode_twobyte(buf, num_chars)
        } else {
            pyunicode_onebyte(buf, num_chars)
        }
    }
}

#[cfg(not(feature = "avx512"))]
#[inline(always)]
pub fn unicode_from_str(buf: &str) -> *mut pyo3_ffi::PyObject {
    if unlikely!(buf.is_empty()) {
        return use_immortal!(crate::typeref::EMPTY_UNICODE);
    }
    str_impl_kind_scalar(buf)
}

#[cfg(not(feature = "avx512"))]
pub fn set_str_create_fn() {}
