// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

use crate::ffi::{BorrowedPyRef, PyDictRef, PyObject};

pub(crate) struct PyDictIterator {
    ob: PyDictRef,
    idx: usize,
    len: usize,
    pos: isize,
    next_key: *mut PyObject,
    next_value: *mut PyObject,
}

impl PyDictIterator {
    #[inline]
    pub fn from_dict(ob: PyDictRef) -> Self {
        let len = ob.len();
        Self {
            ob: ob,
            idx: 0,
            len: len,
            pos: 0,
            next_key: core::ptr::null_mut(),
            next_value: core::ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Iterator for PyDictIterator {
    type Item = (BorrowedPyRef, BorrowedPyRef);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.len() {
            return None;
        }

        unsafe {
            crate::ffi::PyDict_Next(
                self.ob.as_ptr(),
                &raw mut self.pos,
                &raw mut self.next_key,
                &raw mut self.next_value,
            );
        }

        self.idx += 1;
        Some((
            BorrowedPyRef::from_ptr(self.next_key),
            BorrowedPyRef::from_ptr(self.next_value),
        ))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for PyDictIterator {
    fn len(&self) -> usize {
        self.len()
    }
}

impl core::iter::FusedIterator for PyDictIterator {}

#[cfg(feature = "trusted_len")]
unsafe impl core::iter::TrustedLen for PyDictIterator {}
