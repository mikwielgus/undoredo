// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "std")]
mod std;

// No feature for alloc because it would be always enabled anyway.
mod alloc;

#[cfg(feature = "stable-vec")]
mod stable_vec;

#[cfg(feature = "thunderdome")]
mod thunderdome;
