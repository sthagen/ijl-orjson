// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

use crate::serialize::{
    error::SerializeError,
    writer::{BytesWriter, JsonWriter},
};

use crate::ffi::{PyFragmentRef, PyFragmentRefError};

#[cold]
#[inline(never)]
pub(crate) fn serialize_fragment(
    writer: &mut BytesWriter,
    ob: PyFragmentRef,
) -> Result<(), SerializeError> {
    match ob.value() {
        Ok(buffer) => {
            writer.reserve(buffer.len() + 64);
            writer.put_slice(buffer);
            Ok(())
        }
        Err(PyFragmentRefError::InvalidStr) => Err(SerializeError::InvalidStr),
        Err(PyFragmentRefError::InvalidFragment) => Err(SerializeError::InvalidFragment),
    }
}
