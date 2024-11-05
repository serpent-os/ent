// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::{fs, path::Path};

use super::{monitoring::Monitoring, ParserRegistration, Recipe, RecipeError, RecipeParser};

/// Parser implementation for YPKG recipe files
struct Parser {}

/// Structure representing a YPKG recipe with name and version fields
#[derive(serde::Deserialize)]
struct YpkgRecipe {
    /// Package name
    name: String,
    /// Package version
    version: String,
}

impl RecipeParser for Parser {
    /// Parses a YPKG recipe file at the given path and returns a Recipe
    ///
    /// # Arguments
    /// * `recipe` - Path to the YPKG recipe file to parse
    ///
    /// # Returns
    /// * `Result<Recipe, RecipeError>` - Parsed Recipe or error if parsing fails
    fn parse(&self, recipe: &Path) -> Result<Recipe, RecipeError> {
        // Read and parse main recipe file
        let s = fs::read_to_string(recipe)
            .map_err(|_| RecipeError::InvalidRecipe(recipe.display().to_string()))?;

        let p: YpkgRecipe = serde_yaml::from_str(&s)
            .map_err(|_| RecipeError::InvalidRecipe(recipe.display().to_string()))?;

        // Look for adjacent monitoring file
        let adjacent_monitor = ["monitoring.yaml", "monitoring.yml"]
            .iter()
            .map(|name| recipe.with_file_name(name))
            .find(|path| path.exists());

        // Parse monitoring file if it exists
        let monitoring = match adjacent_monitor {
            Some(path) => {
                let s = fs::read_to_string(&path)
                    .map_err(|_| RecipeError::InvalidRecipe(path.display().to_string()))?;
                Monitoring::from_str(&s).ok()
            }
            None => None,
        };

        Ok(Recipe {
            name: p.name,
            version: p.version,
            monitoring,
        })
    }
}

// Register the YPKG recipe parser with the inventory system
inventory::submit! {
    ParserRegistration {
        name: "ypkg_recipe",
        parser: || Box::new(Parser {}),
        pattern: &["*/package.yml"],
    }
}
