// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

#[derive(Clone)]
#[repr(transparent)]
pub(crate) struct BorrowedPyRef {
    ptr: core::ptr::NonNull<pyo3_ffi::PyObject>,
}

unsafe impl Send for BorrowedPyRef {}
unsafe impl Sync for BorrowedPyRef {}

impl PartialEq for BorrowedPyRef {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl BorrowedPyRef {
    #[inline]
    pub fn from_ptr(ptr: *mut pyo3_ffi::PyObject) -> Self {
        debug_assert!(!ptr.is_null());
        Self::from_borrowed_ptr(ptr)
    }

    #[inline]
    pub fn from_borrowed_ptr(ptr: *mut pyo3_ffi::PyObject) -> Self {
        debug_assert!(!ptr.is_null());
        Self { ptr: nonnull!(ptr) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut pyo3_ffi::PyObject {
        self.ptr.as_ptr()
    }
}
