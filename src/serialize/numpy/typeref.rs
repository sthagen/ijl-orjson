// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2020-2026)

use crate::ffi::PyTypeObject;
use crate::typeref::{NUMPY_TYPES, load_numpy_types};

#[cold]
pub(crate) fn is_numpy_scalar(ob_type: *mut PyTypeObject) -> bool {
    let numpy_types = unsafe { NUMPY_TYPES.get_or_init(load_numpy_types) };
    if numpy_types.is_none() {
        false
    } else {
        let scalar_types = unsafe { numpy_types.unwrap().as_ref() };
        core::ptr::eq(ob_type, scalar_types.float64)
            || core::ptr::eq(ob_type, scalar_types.float32)
            || core::ptr::eq(ob_type, scalar_types.float16)
            || core::ptr::eq(ob_type, scalar_types.int64)
            || core::ptr::eq(ob_type, scalar_types.int16)
            || core::ptr::eq(ob_type, scalar_types.int32)
            || core::ptr::eq(ob_type, scalar_types.int8)
            || core::ptr::eq(ob_type, scalar_types.uint64)
            || core::ptr::eq(ob_type, scalar_types.uint32)
            || core::ptr::eq(ob_type, scalar_types.uint8)
            || core::ptr::eq(ob_type, scalar_types.uint16)
            || core::ptr::eq(ob_type, scalar_types.bool_)
            || core::ptr::eq(ob_type, scalar_types.datetime64)
    }
}

#[cold]
pub(crate) fn is_numpy_array(ob_type: *mut PyTypeObject) -> bool {
    let numpy_types = unsafe { NUMPY_TYPES.get_or_init(load_numpy_types) };
    if numpy_types.is_none() {
        false
    } else {
        let scalar_types = unsafe { numpy_types.unwrap().as_ref() };
        unsafe { core::ptr::eq(ob_type, scalar_types.array) }
    }
}
