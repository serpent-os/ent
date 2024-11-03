// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use super::{monitoring::Monitoring, ParserRegistration, Recipe, RecipeParser};

struct Parser {}

#[derive(serde::Deserialize)]
struct YpkgRecipe {
    name: String,
    version: String,
}

impl RecipeParser for Parser {
    fn parse(&self, recipe: &std::path::Path) -> Result<Recipe, super::RecipeError> {
        let s = std::fs::read_to_string(recipe).map_err(|_| super::RecipeError::InvalidRecipe)?;
        let p: YpkgRecipe =
            serde_yaml::from_str(&s).map_err(|_| super::RecipeError::InvalidRecipe)?;

        let mut adjacent_monitor = recipe.with_file_name("monitoring.yaml");
        if !adjacent_monitor.exists() {
            adjacent_monitor = recipe.with_file_name("monitoring.yml");
        }

        let monitoring = if adjacent_monitor.exists() {
            let s = std::fs::read_to_string(&adjacent_monitor)
                .map_err(|_| super::RecipeError::InvalidRecipe)?;
            match Monitoring::from_str(&s) {
                Ok(m) => Some(m),
                Err(_) => None,
            }
        } else {
            None
        };

        let r = Recipe {
            name: p.name,
            version: p.version,
            monitoring,
        };
        Ok(r)
    }
}

inventory::submit! {
    ParserRegistration {
        name: "ypkg_recipe",
        parser: || Box::new(Parser {}),
        pattern: &["*/package.yml"],
    }
}
