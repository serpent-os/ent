// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! API for recipe parsers exposed via `inventory` crate.

use std::path::Path;

use thiserror::Error;

use super::{monitoring, Recipe};

// This is the error type that all parsers must return
#[derive(Debug, Error)]
pub enum RecipeError {
    #[error("Recipe is invalid")]
    InvalidRecipe,

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Monitoring data is invalid")]
    InvalidMonitoring(#[from] monitoring::Error),

    #[error("Recipe is unsupported")]
    UnsupportedRecipe,
}

// This is the trait that all parsers must implement
pub trait RecipeParser {
    // This function is used to parse the recipe
    fn parse(&self, recipe: &Path) -> Result<Recipe, RecipeError>;
}

// This is the registration struct for the parsers
// It is used to register the parsers with the inventory crate
#[derive(Debug)]
pub struct ParserRegistration {
    // The name of the parser
    pub name: &'static str,

    // The function to create a new instance of the parser
    pub parser: fn() -> Box<dyn RecipeParser>,

    // The suffixes that the parser can handle
    pub pattern: &'static [&'static str],
}

inventory::collect!(ParserRegistration);
