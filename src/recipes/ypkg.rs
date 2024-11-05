// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::{fs, path::Path};

use super::{monitoring::Monitoring, ParserRegistration, Recipe, RecipeError, RecipeParser};

struct Parser {}

#[derive(serde::Deserialize)]
struct YpkgRecipe {
    name: String,
    version: String,
}

impl RecipeParser for Parser {
    fn parse(&self, recipe: &Path) -> Result<Recipe, RecipeError> {
        let s = fs::read_to_string(recipe)
            .map_err(|_| RecipeError::InvalidRecipe(recipe.display().to_string()))?;

        let p: YpkgRecipe = serde_yaml::from_str(&s)
            .map_err(|_| RecipeError::InvalidRecipe(recipe.display().to_string()))?;

        let adjacent_monitor = ["monitoring.yaml", "monitoring.yml"]
            .iter()
            .map(|name| recipe.with_file_name(name))
            .find(|path| path.exists());

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

inventory::submit! {
    ParserRegistration {
        name: "ypkg_recipe",
        parser: || Box::new(Parser {}),
        pattern: &["*/package.yml"],
    }
}
