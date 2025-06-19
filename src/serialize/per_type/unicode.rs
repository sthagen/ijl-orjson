// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::serialize::error::SerializeError;
use crate::str::{PyStr, PyStrSubclass};

use serde::ser::{Serialize, Serializer};

#[repr(transparent)]
pub(crate) struct StrSerializer {
    ptr: *mut pyo3_ffi::PyObject,
}

impl StrSerializer {
    pub fn new(ptr: *mut pyo3_ffi::PyObject) -> Self {
        StrSerializer { ptr: ptr }
    }
}

impl Serialize for StrSerializer {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match unsafe { PyStr::from_ptr_unchecked(self.ptr).to_str() } {
            Some(uni) => serializer.serialize_str(uni),
            None => err!(SerializeError::InvalidStr),
        }
    }
}

#[repr(transparent)]
pub(crate) struct StrSubclassSerializer {
    ptr: *mut pyo3_ffi::PyObject,
}

impl StrSubclassSerializer {
    pub fn new(ptr: *mut pyo3_ffi::PyObject) -> Self {
        StrSubclassSerializer { ptr: ptr }
    }
}

impl Serialize for StrSubclassSerializer {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match unsafe { PyStrSubclass::from_ptr_unchecked(self.ptr).to_str() } {
            Some(uni) => serializer.serialize_str(uni),
            None => err!(SerializeError::InvalidStr),
        }
    }
}
