// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

use crate::ffi::{PyByteArray_AsString, PyByteArray_Size, PyByteArray_Type, PyObject};

pub(crate) enum PyByteArrayRefError {
    NotType,
}

#[derive(Clone)]
#[repr(transparent)]
pub(crate) struct PyByteArrayRef {
    ptr: core::ptr::NonNull<PyObject>,
}

unsafe impl Send for PyByteArrayRef {}
unsafe impl Sync for PyByteArrayRef {}

impl PartialEq for PyByteArrayRef {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl PyByteArrayRef {
    #[inline]
    pub fn from_ptr(ptr: *mut PyObject) -> Result<Self, PyByteArrayRefError> {
        unsafe {
            debug_assert!(!ptr.is_null());
            if crate::ffi::PyObject_Type(ptr) == &raw mut PyByteArray_Type {
                Ok(Self {
                    ptr: core::ptr::NonNull::new_unchecked(ptr),
                })
            } else {
                Err(PyByteArrayRefError::NotType)
            }
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut PyObject {
        self.ptr.as_ptr()
    }

    #[allow(unused)]
    #[inline]
    pub fn as_non_null_ptr(&self) -> core::ptr::NonNull<PyObject> {
        self.ptr
    }

    #[inline]
    pub fn as_bytes(&self) -> &'static [u8] {
        unsafe {
            core::slice::from_raw_parts(
                PyByteArray_AsString(self.as_ptr())
                    .cast::<u8>()
                    .cast_const(),
                crate::util::isize_to_usize(PyByteArray_Size(self.as_ptr())),
            )
        }
    }

    #[inline]
    pub fn as_str(&self) -> Option<&'static str> {
        let buffer = self.as_bytes();
        if !crate::ffi::utf8::is_valid_utf8(buffer) {
            cold_path!();
            None
        } else {
            unsafe { Some(core::str::from_utf8_unchecked(buffer)) }
        }
    }
}
