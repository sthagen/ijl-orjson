// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

use crate::ffi::{BorrowedPyRef, PyListRef};

pub(crate) struct PyListIterator {
    ob: PyListRef,
    idx: usize,
    len: usize,
}

impl PyListIterator {
    #[inline]
    pub fn from_list(ob: PyListRef) -> Self {
        let len = ob.len();
        Self {
            ob: ob,
            idx: 0,
            len: len,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Iterator for PyListIterator {
    type Item = BorrowedPyRef;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.len() {
            return None;
        }
        let value = self.ob.get(self.idx);
        self.idx += 1;
        Some(BorrowedPyRef::from_ptr(value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl ExactSizeIterator for PyListIterator {
    fn len(&self) -> usize {
        self.len()
    }
}

impl core::iter::FusedIterator for PyListIterator {}

#[cfg(feature = "trusted_len")]
unsafe impl core::iter::TrustedLen for PyListIterator {}
