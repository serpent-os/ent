// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Recipe parsing and handling library

mod parser;

pub use parser::*;

mod stone;

// Source recipe details
#[derive(Debug)]
pub struct Recipe {
    pub name: String,
    pub version: String,
}
