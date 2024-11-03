// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use super::{monitoring::Monitoring, ParserRegistration, Recipe, RecipeParser};

struct Parser {}

impl RecipeParser for Parser {
    fn parse(&self, recipe: &std::path::Path) -> Result<Recipe, super::RecipeError> {
        let s = std::fs::read_to_string(recipe).map_err(|_| super::RecipeError::InvalidRecipe)?;
        let p = stone_recipe::from_str(&s).map_err(|_| super::RecipeError::InvalidRecipe)?;

        let adjacent_monitor = recipe.with_file_name("monitoring.yaml");
        let monitoring = if adjacent_monitor.exists() {
            let s = std::fs::read_to_string(&adjacent_monitor)
                .map_err(|_| super::RecipeError::InvalidRecipe)?;
            let m = Monitoring::from_str(&s)?;
            Some(m)
        } else {
            None
        };

        let r = Recipe {
            name: p.source.name,
            version: p.source.version,
            monitoring,
        };
        Ok(r)
    }
}

inventory::submit! {
    ParserRegistration {
        name: "stone_recipe",
        parser: || Box::new(Parser {}),
        pattern: &["*/stone.yaml"],
    }
}
