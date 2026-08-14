// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

pub(crate) unsafe trait JsonWriter {
    fn as_mut_buffer_ptr(&mut self) -> *mut u8;

    unsafe fn advance_mut(&mut self, cnt: usize);

    fn remaining_mut(&self) -> usize;

    fn put_null(&mut self);

    fn put_u8(&mut self, value: u8);

    fn put_bytes(&mut self, val: u8, cnt: usize);

    fn put_slice(&mut self, src: &[u8]);

    fn reserve(&mut self, len: usize);

    fn reserve_minimum(&mut self);

    fn quote(&mut self);
}
