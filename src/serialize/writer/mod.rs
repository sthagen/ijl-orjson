// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2024-2026)

mod byteswriter;
mod container;
mod format_str;
mod half;
mod jsonwriter;
mod num;
mod smallfixedbuffer;
mod str;
mod uuid;

pub(crate) use byteswriter::{BytesWriter, CompactFormatter, IndentFormatter, WriteFormatter};
pub(crate) use container::ContainerSerializer;
pub(crate) use format_str::{format_escaped_str, set_str_formatter_fn};
pub(crate) use half::f16_to_f32;
pub(crate) use jsonwriter::JsonWriter;
pub(crate) use num::{
    write_float32, write_float64, write_integer_i32, write_integer_i64, write_integer_u32,
    write_integer_u64,
};
pub(crate) use smallfixedbuffer::SmallFixedBuffer;
pub(crate) use uuid::format_hyphenated;
