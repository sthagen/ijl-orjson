// SPDX-License-Identifier: (Apache-2.0 OR MIT)
// Copyright ijl (2020-2026), Ben Sully (2021)

use super::datetime::{datetime_into_error, write_numpy_datetime};
use super::item::ItemType;
use crate::ffi::{
    NPY_ARRAY_C_CONTIGUOUS, NPY_ARRAY_NOTSWAPPED, NumpyDatetimeUnit, Py_DECREF, PyArrayInterface,
    PyCapsule, PyObject, PyObject_GetAttr,
};
use crate::opt::Opt;
use crate::serialize::{
    error::SerializeError,
    writer::{
        BytesWriter, JsonWriter, WriteFormatter, f16_to_f32, write_float32, write_float64,
        write_integer_i32, write_integer_i64, write_integer_u32, write_integer_u64,
    },
};
use crate::typeref::ARRAY_STRUCT_STR;
use crate::util::isize_to_usize;
use core::ffi::c_void;

macro_rules! slice {
    ($ptr:expr, $size:expr) => {
        unsafe { core::slice::from_raw_parts($ptr, $size) }
    };
}

pub(crate) enum PyArrayError {
    Malformed,
    NotContiguous,
    NotNativeEndian,
    UnsupportedDataType,
}

// >>> arr = numpy.array([[[1, 2], [3, 4]], [[5, 6], [7, 8]]], numpy.int32)
// >>> arr.ndim
// 3
// >>> arr.shape
// (2, 2, 2)
// >>> arr.strides
// (16, 8, 4)
pub(crate) struct NumpyArray<F> {
    array: *mut PyArrayInterface,
    position: Vec<isize>,
    children: Vec<NumpyArray<F>>,
    depth: usize,
    capsule: *mut PyCapsule,
    kind: ItemType,
    opts: Opt,
    formatter: F,
}

impl<F> NumpyArray<F>
where
    F: WriteFormatter + Clone,
{
    #[cold]
    #[inline(never)]
    #[cfg_attr(feature = "optimize", optimize(size))]
    pub fn new(formatter: F, ptr: *mut PyObject, opts: Opt) -> Result<Self, PyArrayError> {
        let capsule = unsafe { PyObject_GetAttr(ptr, ARRAY_STRUCT_STR) };
        debug_assert!(!capsule.is_null());
        let array = unsafe {
            (*capsule.cast::<PyCapsule>())
                .pointer
                .cast::<PyArrayInterface>()
        };
        debug_assert!(!array.is_null());
        if unsafe { (*array).two != 2 } {
            unsafe {
                Py_DECREF(capsule);
            }
            Err(PyArrayError::Malformed)
        } else if unsafe { (*array).flags } & NPY_ARRAY_C_CONTIGUOUS != NPY_ARRAY_C_CONTIGUOUS {
            unsafe {
                Py_DECREF(capsule);
            }
            Err(PyArrayError::NotContiguous)
        } else if unsafe { (*array).flags } & NPY_ARRAY_NOTSWAPPED != NPY_ARRAY_NOTSWAPPED {
            unsafe {
                Py_DECREF(capsule);
            }
            Err(PyArrayError::NotNativeEndian)
        } else {
            debug_assert!(unsafe { (*array).nd >= 0 });
            #[allow(clippy::cast_sign_loss)]
            let num_dimensions = unsafe { (*array).nd as usize };
            if num_dimensions == 0 {
                unsafe {
                    Py_DECREF(capsule);
                }
                return Err(PyArrayError::UnsupportedDataType);
            }
            match ItemType::find(array, ptr) {
                None => {
                    unsafe {
                        Py_DECREF(capsule);
                    }
                    Err(PyArrayError::UnsupportedDataType)
                }
                Some(kind) => {
                    let mut pyarray = NumpyArray {
                        array: array,
                        position: vec![0; num_dimensions],
                        children: Vec::with_capacity(num_dimensions),
                        depth: 0,
                        capsule: capsule.cast::<PyCapsule>(),
                        kind: kind,
                        opts: opts,
                        formatter: formatter,
                    };
                    if pyarray.dimensions() > 1 {
                        pyarray.build();
                    }
                    Ok(pyarray)
                }
            }
        }
    }

    #[cold]
    #[inline(never)]
    pub fn write(&self, buf: &mut BytesWriter) -> Result<(), SerializeError> {
        buf.reserve_minimum();
        if !(self.depth >= self.dimensions() || self.shape()[self.depth] != 0) {
            cold_path!();
            buf.put_slice(b"[]");
            Ok(())
        } else if !self.children.is_empty() {
            cold_path!();
            F::array_open(buf);
            for (idx, array) in self.children.iter().enumerate() {
                if idx > 0 {
                    F::item_separator(buf);
                }
                let _ = array.write(buf);
            }
            F::array_close(buf);
            Ok(())
        } else {
            let len = self.num_items();
            match self.kind {
                ItemType::F64 => {
                    NumpyF64Array::new(slice!(self.data().cast::<f64>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::F32 => {
                    NumpyF32Array::new(slice!(self.data().cast::<f32>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::F16 => {
                    NumpyF16Array::new(slice!(self.data().cast::<u16>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::U64 => {
                    NumpyU64Array::new(slice!(self.data().cast::<u64>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::U32 => {
                    NumpyU32Array::new(slice!(self.data().cast::<u32>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::U16 => {
                    NumpyU16Array::new(slice!(self.data().cast::<u16>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::U8 => {
                    NumpyU8Array::new(slice!(self.data().cast::<u8>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::I64 => {
                    NumpyI64Array::new(slice!(self.data().cast::<i64>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::I32 => {
                    NumpyI32Array::new(slice!(self.data().cast::<i32>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::I16 => {
                    NumpyI16Array::new(slice!(self.data().cast::<i16>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::I8 => {
                    NumpyI8Array::new(slice!(self.data().cast::<i8>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::BOOL => {
                    NumpyBoolArray::new(slice!(self.data().cast::<u8>(), len)).write::<F>(buf);
                    Ok(())
                }
                ItemType::DATETIME64(unit) => NumpyDatetime64Array::new(
                    slice!(self.data().cast::<i64>(), len),
                    unit,
                    self.opts,
                )
                .write::<F>(buf),
            }
        }
    }

    fn child_from_parent(&self, position: Vec<isize>, num_children: usize) -> Self {
        let mut arr = NumpyArray {
            array: self.array,
            position: position,
            children: Vec::with_capacity(num_children),
            depth: self.depth + 1,
            capsule: self.capsule,
            kind: self.kind,
            opts: self.opts,
            formatter: self.formatter.clone(),
        };
        arr.build();
        arr
    }

    pub fn build(&mut self) {
        if self.depth < self.dimensions() - 1 {
            for i in 0..self.shape()[self.depth] {
                let mut position: Vec<isize> = self.position.clone();
                position[self.depth] = i;
                let num_children: usize = if self.depth < self.dimensions() - 2 {
                    isize_to_usize(self.shape()[self.depth + 1])
                } else {
                    0
                };
                self.children
                    .push(self.child_from_parent(position, num_children));
            }
        }
    }

    #[inline(always)]
    pub fn data(&self) -> *const c_void {
        let offset = self
            .strides()
            .iter()
            .zip(self.position.iter().copied())
            .take(self.depth)
            .map(|(a, b)| a * b)
            .sum::<isize>();
        unsafe { (*self.array).data.offset(offset) }
    }

    pub fn num_items(&self) -> usize {
        isize_to_usize(self.shape()[self.shape().len() - 1])
    }

    pub fn dimensions(&self) -> usize {
        #[allow(clippy::cast_sign_loss)]
        unsafe {
            (*self.array).nd as usize
        }
    }

    pub fn shape(&self) -> &[isize] {
        slice!((*self.array).shape.cast_const(), self.dimensions())
    }

    pub fn strides(&self) -> &[isize] {
        slice!((*self.array).strides.cast_const(), self.dimensions())
    }
}

impl<F> Drop for NumpyArray<F> {
    fn drop(&mut self) {
        if self.depth == 0 {
            unsafe {
                Py_DECREF(self.array.cast::<PyObject>());
                Py_DECREF(self.capsule.cast::<PyObject>());
            };
        }
    }
}

#[repr(transparent)]
pub(crate) struct NumpyF64Array<'a> {
    pub data: &'a [f64],
}

impl<'a> NumpyF64Array<'a> {
    pub const fn new(data: &'a [f64]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<f64>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_float64(buf, each);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyF32Array<'a> {
    pub data: &'a [f32],
}

impl<'a> NumpyF32Array<'a> {
    pub const fn new(data: &'a [f32]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<f32>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_float32(buf, each);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyF16Array<'a> {
    pub data: &'a [u16],
}

impl<'a> NumpyF16Array<'a> {
    pub const fn new(data: &'a [u16]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<u16>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_float32(buf, f16_to_f32(each));
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyU64Array<'a> {
    pub data: &'a [u64],
}

impl<'a> NumpyU64Array<'a> {
    pub const fn new(data: &'a [u64]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<u64>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_u64(buf, each);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyU32Array<'a> {
    pub data: &'a [u32],
}

impl<'a> NumpyU32Array<'a> {
    pub const fn new(data: &'a [u32]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<u32>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_u32(buf, each);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyU16Array<'a> {
    pub data: &'a [u16],
}

impl<'a> NumpyU16Array<'a> {
    pub const fn new(data: &'a [u16]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<u16>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_u32(buf, each as u32);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyU8Array<'a> {
    pub data: &'a [u8],
}

impl<'a> NumpyU8Array<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<u8>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_u32(buf, each as u32);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyI64Array<'a> {
    pub data: &'a [i64],
}

impl<'a> NumpyI64Array<'a> {
    pub const fn new(data: &'a [i64]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<i64>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_i64(buf, each);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyI32Array<'a> {
    pub data: &'a [i32],
}

impl<'a> NumpyI32Array<'a> {
    pub const fn new(data: &'a [i32]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<i32>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_i32(buf, each);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyI16Array<'a> {
    pub data: &'a [i16],
}

impl<'a> NumpyI16Array<'a> {
    pub const fn new(data: &'a [i16]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<i16>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_i32(buf, each as i32);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyI8Array<'a> {
    pub data: &'a [i8],
}

impl<'a> NumpyI8Array<'a> {
    pub const fn new(data: &'a [i8]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<i8>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            write_integer_i32(buf, each as i32);
        }
        F::array_close(buf);
    }
}

#[repr(transparent)]
pub(crate) struct NumpyBoolArray<'a> {
    pub data: &'a [u8],
}

impl<'a> NumpyBoolArray<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) {
        F::reserve_array(buf, self.data.len(), core::mem::size_of::<u8>());
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }
            buf.put_bool(each != 0);
        }
        F::array_close(buf);
    }
}

pub(crate) struct NumpyDatetime64Array<'a> {
    pub data: &'a [i64],
    pub unit: NumpyDatetimeUnit,
    pub opts: Opt,
}

impl<'a> NumpyDatetime64Array<'a> {
    pub const fn new(data: &'a [i64], unit: NumpyDatetimeUnit, opts: Opt) -> Self {
        Self { data, unit, opts }
    }

    #[cold]
    #[inline(never)]
    pub fn write<F: WriteFormatter>(&self, buf: &mut BytesWriter) -> Result<(), SerializeError> {
        F::reserve_array(buf, self.data.len(), 40);
        F::array_open(buf);
        let mut repeating = false;
        for &each in self.data.iter().fuse() {
            if repeating {
                F::item_separator(buf);
            } else {
                repeating = true;
            }

            write_numpy_datetime(
                &self
                    .unit
                    .datetime(each, self.opts)
                    .map_err(datetime_into_error)?,
                buf,
            )
        }
        F::array_close(buf);
        Ok(())
    }
}
