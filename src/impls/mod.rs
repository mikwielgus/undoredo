// SPDX-FileCopyrightText: 2025 undoredo Developers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "std")]
mod std;

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "stable-vec")]
mod stable_vec;

#[cfg(feature = "thunderdome")]
mod thunderdome;
