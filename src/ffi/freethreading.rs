// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2026)

#[allow(unused)]
unsafe extern "C" {
    pub fn PyMutex_IsLocked(m: *mut pyo3_ffi::PyMutex) -> core::ffi::c_int;

    pub fn PyCriticalSection_BeginMutex(
        c: *mut pyo3_ffi::PyCriticalSection,
        m: *mut pyo3_ffi::PyMutex,
    ) -> core::ffi::c_int;

    pub fn PyCriticalSection_End(c: *mut pyo3_ffi::PyCriticalSection);
}
