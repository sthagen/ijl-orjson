// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2023-2026)

use crate::serialize::writer::JsonWriter;
use core::mem::MaybeUninit;

const BUFFER_LENGTH: usize = 64 - core::mem::size_of::<usize>();

/// For use to serialize fixed-size UUIDs and DateTime.
#[repr(align(64))]
pub(crate) struct SmallFixedBuffer {
    idx: usize,
    bytes: [MaybeUninit<u8>; BUFFER_LENGTH],
}

impl SmallFixedBuffer {
    #[inline]
    pub fn new() -> Self {
        Self {
            idx: 0,
            bytes: [MaybeUninit::<u8>::uninit(); BUFFER_LENGTH],
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        (&raw const self.bytes).cast::<u8>()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.idx
    }

    #[allow(clippy::inherent_to_string)]
    #[inline]
    pub fn to_string(&self) -> String {
        String::from(str_from_slice!(self.as_ptr(), self.len()))
    }
}

unsafe impl JsonWriter for SmallFixedBuffer {
    #[inline(always)]
    fn as_mut_buffer_ptr(&mut self) -> *mut u8 {
        unsafe { (&raw mut self.bytes).cast::<u8>().add(self.idx) }
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.idx += cnt;
    }

    #[inline]
    fn remaining_mut(&self) -> usize {
        BUFFER_LENGTH - self.idx
    }

    #[inline]
    fn put_null(&mut self) {
        self.put_slice(b"null");
    }

    #[inline]
    fn put_u8(&mut self, value: u8) {
        debug_assert!(self.remaining_mut() > 8);
        unsafe {
            core::ptr::write((&raw mut self.bytes).cast::<u8>().add(self.idx), value);
            self.advance_mut(1);
        };
    }

    #[inline]
    fn put_bytes(&mut self, val: u8, cnt: usize) {
        debug_assert!(self.remaining_mut() > cnt);
        debug_assert!(self.remaining_mut() > 8);
        unsafe {
            core::ptr::write_bytes((&raw mut self.bytes).cast::<u8>().add(self.idx), val, cnt);
            self.advance_mut(cnt);
        };
    }

    #[inline]
    fn put_slice(&mut self, src: &[u8]) {
        debug_assert!(self.remaining_mut() > src.len());
        unsafe {
            let len = src.len();
            core::ptr::copy_nonoverlapping(
                src.as_ptr(),
                (&raw mut self.bytes).cast::<u8>().add(self.idx),
                len,
            );
            self.advance_mut(len);
        }
    }

    fn reserve(&mut self, _: usize) {
        unimplemented!();
    }

    fn reserve_minimum(&mut self) {
        unimplemented!();
    }

    #[inline]
    fn quote(&mut self) {
        self.put_u8(b'"');
    }
}
