// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use super::{monitoring::Monitoring, ParserRegistration, Recipe, RecipeError, RecipeParser};
use std::{fs, path::Path};

/// A parser implementation for stone recipe files that parses recipe files and any associated
/// monitoring configuration.
#[derive(Default)]
struct Parser {}

impl RecipeParser for Parser {
    /// Parses a stone recipe file and its optional monitoring configuration.
    ///
    /// # Arguments
    ///
    /// * `recipe` - Path to the stone recipe file to parse
    ///
    /// # Returns
    ///
    /// Returns a Result containing either:
    /// - A populated Recipe struct with recipe and monitoring data
    /// - A RecipeError if parsing fails
    ///
    /// # Notes
    ///
    /// The parser will look for an additional monitoring.yaml file in the same directory
    /// as the recipe file. If found, its contents will be parsed and included in the
    /// resulting Recipe struct.
    fn parse(&self, recipe: &Path) -> Result<Recipe, RecipeError> {
        // Parse the main recipe file
        let recipe_contents = fs::read_to_string(recipe)
            .map_err(|_| {
                RecipeError::InvalidRecipe(recipe.to_str().unwrap_or_default().to_string())
            })
            .unwrap_or_default();

        let parsed_recipe = stone_recipe::from_str(&recipe_contents).map_err(|_| {
            RecipeError::InvalidRecipe(recipe.to_str().unwrap_or_default().to_string())
        })?;

        // Check for and parse optional monitoring config
        let adjacent_monitor = recipe.with_file_name("monitoring.yaml");
        let monitoring = if adjacent_monitor.exists() {
            let monitoring_contents = fs::read_to_string(&adjacent_monitor)
                .map_err(|_| RecipeError::InvalidRecipe(adjacent_monitor.display().to_string()))
                .unwrap_or_default();
            Some(Monitoring::from_str(&monitoring_contents).map_err(|e| {
                RecipeError::InvalidMonitoring(e, adjacent_monitor.display().to_string())
            })?)
        } else {
            None
        };

        Ok(Recipe {
            name: parsed_recipe.source.name,
            version: parsed_recipe.source.version,
            monitoring,
        })
    }
}

// Register the stone recipe parser with the recipe parser inventory
inventory::submit! {
    ParserRegistration {
        name: "stone_recipe",
        parser: || Box::new(Parser::default()),
        pattern: &["*/stone.yaml"],
    }
}
