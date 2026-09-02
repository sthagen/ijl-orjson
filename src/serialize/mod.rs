// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2022-2026)

pub(crate) mod datetime;
mod error;
mod fragment;
mod non_str;
mod num;
mod numpy;
mod obtype;
mod state;
mod uuid;
pub(crate) mod writer;

pub(crate) use error::SerializeError;
pub(crate) use writer::{
    CompactFormatter, ContainerSerializer, IndentFormatter, set_str_formatter_fn,
};

pub(crate) fn serialize(
    ptr: *mut pyo3_ffi::PyObject,
    default: Option<core::ptr::NonNull<pyo3_ffi::PyObject>>,
    opts: crate::opt::Opt,
) -> Result<core::ptr::NonNull<pyo3_ffi::PyObject>, SerializeError> {
    if opt_disabled!(opts, crate::opt::INDENT_2) {
        let mut serializer =
            ContainerSerializer::new(CompactFormatter, state::SerializerState::new(opts), default);
        serializer.write(ptr)
    } else {
        let mut serializer =
            ContainerSerializer::new(IndentFormatter, state::SerializerState::new(opts), default);
        serializer.write(ptr)
    }
}
