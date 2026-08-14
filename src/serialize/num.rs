// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2019-2026)

use crate::ffi::{PyFloatRef, PyIntRef};
use crate::serialize::{
    error::SerializeError,
    writer::{BytesWriter, write_float64, write_integer_i64, write_integer_u64},
};

#[cfg(feature = "inline_int")]
use crate::ffi::PyIntKind;
#[cfg(feature = "inline_int")]
use crate::serialize::writer::{write_integer_i32, write_integer_u32};

// https://tools.ietf.org/html/rfc7159#section-6
// "[-(2**53)+1, (2**53)-1]"
const STRICT_INT_MIN: i64 = -9007199254740991;
const STRICT_INT_MAX: i64 = 9007199254740991;

#[cfg(feature = "inline_int")]
#[inline(always)]
pub(crate) fn serialize_int(
    writer: &mut BytesWriter,
    ob: PyIntRef,
    strict_integer: bool,
) -> Result<(), SerializeError> {
    unsafe {
        match ob.kind() {
            PyIntKind::I32 => {
                write_integer_i32(writer, ob.as_i32());
                Ok(())
            }
            PyIntKind::U32 => {
                write_integer_u32(writer, ob.as_u32());
                Ok(())
            }
            PyIntKind::I64 => {
                let value = ob.as_i64().map_err(|_| SerializeError::Integer64Bits)?;
                if strict_integer && !(STRICT_INT_MIN..=STRICT_INT_MAX).contains(&value) {
                    cold_path!();
                    return Err(SerializeError::Integer53Bits);
                }
                write_integer_i64(writer, value);
                Ok(())
            }
            PyIntKind::U64 => {
                let value = ob.as_u64().map_err(|_| SerializeError::Integer64Bits)?;
                if strict_integer && value > STRICT_INT_MAX as u64 {
                    cold_path!();
                    return Err(SerializeError::Integer53Bits);
                }
                write_integer_u64(writer, value);
                Ok(())
            }
        }
    }
}

#[cfg(not(feature = "inline_int"))]
#[inline(always)]
pub(crate) fn serialize_int(
    writer: &mut BytesWriter,
    ob: PyIntRef,
    strict_integer: bool,
) -> Result<(), SerializeError> {
    unsafe {
        match ob.as_i64() {
            Ok(value) => {
                if strict_integer && !(STRICT_INT_MIN..=STRICT_INT_MAX).contains(&value) {
                    cold_path!();
                    return Err(SerializeError::Integer53Bits);
                }
                write_integer_i64(writer, value);
                Ok(())
            }
            Err(_) => match ob.as_u64() {
                Ok(value) => {
                    if strict_integer && value > STRICT_INT_MAX as u64 {
                        cold_path!();
                        return Err(SerializeError::Integer53Bits);
                    }
                    write_integer_u64(writer, value);
                    Ok(())
                }
                Err(_) => Err(SerializeError::Integer64Bits),
            },
        }
    }
}

pub(crate) fn serialize_float(writer: &mut BytesWriter, ob: PyFloatRef) {
    write_float64(writer, ob.value());
}
