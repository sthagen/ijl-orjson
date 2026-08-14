// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

use crate::ffi::{BorrowedPyRef, PyTupleRef};
use core::iter::FusedIterator;

pub(crate) struct PyTupleIterator {
    ob: PyTupleRef,
    idx: usize,
    len: usize,
}

impl PyTupleIterator {
    #[inline]
    pub fn from_tuple(ob: PyTupleRef) -> Self {
        let len = ob.len();
        Self {
            ob: ob,
            idx: 0,
            len: len,
        }
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
}

impl Iterator for PyTupleIterator {
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

impl ExactSizeIterator for PyTupleIterator {
    fn len(&self) -> usize {
        self.len()
    }
}

impl FusedIterator for PyTupleIterator {}

#[cfg(feature = "trusted_len")]
unsafe impl core::iter::TrustedLen for PyTupleIterator {}
