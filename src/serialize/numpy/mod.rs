// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

mod array;
mod datetime;
mod item;
mod scalar;
mod typeref;

pub(crate) use array::{NumpyArray, PyArrayError};
pub(crate) use scalar::NumpyScalar;
pub(crate) use typeref::{is_numpy_array, is_numpy_scalar};
