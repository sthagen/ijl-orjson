// SPDX-License-Identifier: MPL-2.0
// Copyright ijl (2024-2026)

use crate::opt::Opt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub(crate) struct SerializerState {
    state: u32,
}

impl SerializerState {
    #[inline(always)]
    pub fn new(opts: Opt) -> Self {
        debug_assert!(opts < u32::from(u16::MAX));
        Self { state: opts }
    }

    #[inline(always)]
    pub const fn opts(self) -> u32 {
        self.state
    }

    #[inline(always)]
    pub const fn is_enabled(self, var: Opt) -> bool {
        self.state & var != 0
    }

    #[inline(always)]
    pub const fn is_disabled(self, var: Opt) -> bool {
        self.state & var == 0
    }
}
