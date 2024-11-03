// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::{collections::HashMap, path::Path};

use clap::{Parser, Subcommand};
use ent::recipes::{self, ParserRegistration, Recipe, RecipeError};
use glob::Pattern;

/// A simple CLI tool to check for working with recipe trees
#[derive(Parser)]
#[command(name = "cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Refresh the cache
    Refresh,
    /// Check for updates or security status
    Check {
        #[command(subcommand)]
        check_command: CheckCommands,
    },
}

#[derive(Subcommand)]
enum CheckCommands {
    /// Check for updates
    Updates,
    /// Check for security status
    Security,
}

// This function scans the directory for recipes and parses them
fn scan_dir(
    root: impl AsRef<Path>,
    globs: &HashMap<Pattern, &&ParserRegistration>,
) -> Result<Vec<recipes::Recipe>, recipes::RecipeError> {
    let root = root.as_ref();
    let mut ret = vec![];

    for entry in root.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            ret.extend(scan_dir(&path, globs)?);
        } else {
            for (pattern, parser) in globs {
                if pattern.matches_path(&path) {
                    let parser = (parser.parser)();
                    let r = parser.parse(&path).unwrap();
                    ret.push(r);
                }
            }
        }
    }

    Ok(ret)
}

// This function scans the recipes in the current directory
fn scan_recipes(root: impl AsRef<Path>) -> Result<Vec<Recipe>, RecipeError> {
    let registry = inventory::iter::<ParserRegistration>
        .into_iter()
        .map(|p| (p.name, p))
        .collect::<HashMap<_, _>>();

    let glob_patterns = registry
        .values()
        .flat_map(|p| {
            p.pattern
                .iter()
                .map(move |&s| (Pattern::new(s).unwrap(), p))
        })
        .collect::<HashMap<_, _>>();

    let scanned = scan_dir(root, &glob_patterns)?;
    Ok(scanned)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Refresh => {
            todo!("Implement refresh");
        }
        Commands::Check { check_command } => match check_command {
            CheckCommands::Updates => {
                println!("Checking for updates...");
                let recipes = scan_recipes(".")?;
                eprintln!("{:?}", recipes);
            }
            CheckCommands::Security => {
                todo!("Implement security check");
            }
        },
    }

    Ok(())
}
