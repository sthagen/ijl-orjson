// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2020-2026)

use super::datetime::{datetime_into_error, write_numpy_datetime};
use crate::ffi::{
    NumpyBool, NumpyDatetime64, NumpyDatetimeUnit, NumpyFloat16, NumpyFloat32, NumpyFloat64,
    NumpyInt8, NumpyInt16, NumpyInt32, NumpyInt64, NumpyUint8, NumpyUint16, NumpyUint32,
    NumpyUint64, PyObject,
};
use crate::opt::Opt;
use crate::serialize::{
    error::SerializeError,
    writer::{
        BytesWriter, JsonWriter, f16_to_f32, write_float32, write_float64, write_integer_i32,
        write_integer_i64, write_integer_u32, write_integer_u64,
    },
};
use crate::typeref::{NUMPY_TYPES, load_numpy_types};

pub(crate) struct NumpyScalar {
    pub ptr: *mut PyObject,
    pub opts: Opt,
}

impl NumpyScalar {
    pub const fn new(ptr: *mut PyObject, opts: Opt) -> Self {
        NumpyScalar { ptr, opts }
    }

    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) -> Result<(), SerializeError> {
        unsafe {
            let ob_type = crate::ffi::PyObject_Type(self.ptr);
            let scalar_types =
                unsafe { NUMPY_TYPES.get_or_init(load_numpy_types).unwrap().as_ref() };
            if core::ptr::eq(ob_type, scalar_types.float64) {
                (*(self.ptr.cast::<NumpyFloat64>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.float32) {
                (*(self.ptr.cast::<NumpyFloat32>())).write(buf)
            } else if core::ptr::eq(ob_type, scalar_types.float16) {
                (*(self.ptr.cast::<NumpyFloat16>())).write(buf)
            } else if core::ptr::eq(ob_type, scalar_types.int64) {
                (*(self.ptr.cast::<NumpyInt64>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.int32) {
                (*(self.ptr.cast::<NumpyInt32>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.int16) {
                (*(self.ptr.cast::<NumpyInt16>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.int8) {
                (*(self.ptr.cast::<NumpyInt8>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.uint64) {
                (*(self.ptr.cast::<NumpyUint64>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.uint32) {
                (*(self.ptr.cast::<NumpyUint32>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.uint16) {
                (*(self.ptr.cast::<NumpyUint16>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.uint8) {
                (*(self.ptr.cast::<NumpyUint8>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.bool_) {
                (*(self.ptr.cast::<NumpyBool>())).write(buf);
            } else if core::ptr::eq(ob_type, scalar_types.datetime64) {
                write_numpy_datetime(
                    &NumpyDatetimeUnit::from_pyobject(self.ptr)
                        .datetime((&*self.ptr.cast::<NumpyDatetime64>()).value, self.opts)
                        .map_err(datetime_into_error)?,
                    buf,
                )
            } else {
                unreachable_unchecked!()
            }
            Ok(())
        }
    }
}

impl NumpyInt8 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_i32(buf, self.value as i32);
    }
}

impl NumpyInt16 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_i32(buf, self.value as i32);
    }
}

impl NumpyInt32 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_i32(buf, self.value);
    }
}

impl NumpyInt64 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_i64(buf, self.value);
    }
}

impl NumpyUint8 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_u32(buf, self.value as u32);
    }
}

impl NumpyUint16 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_u32(buf, self.value as u32);
    }
}

impl NumpyUint32 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_u32(buf, self.value);
    }
}

impl NumpyUint64 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        write_integer_u64(buf, self.value);
    }
}

impl NumpyFloat16 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        buf.reserve_minimum();
        write_float32(buf, f16_to_f32(self.value));
    }
}

impl NumpyFloat32 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        buf.reserve_minimum();
        write_float32(buf, self.value);
    }
}

impl NumpyFloat64 {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        buf.reserve_minimum();
        write_float64(buf, self.value);
    }
}

impl NumpyBool {
    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) {
        buf.reserve_minimum();
        buf.put_bool(self.value);
    }
}
