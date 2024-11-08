// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::{collections::HashMap, path::Path};

use clap::{Parser, Subcommand};
use colored::Colorize;
use ent::{
    data,
    recipes::{self, ParserRegistration, Recipe, RecipeError},
};
use futures::StreamExt;
use glob::Pattern;
use indicatif::ProgressBar;

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

    for entry in root.read_dir()?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            ret.extend(scan_dir(&path, globs)?);
        } else {
            for (pattern, parser) in globs {
                if pattern.matches_path(&path) {
                    let parser = (parser.parser)();
                    let r = parser.parse(&path)?;
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

/// A required update for CLI rendering
#[derive(Debug)]
pub struct RequiredUpdate {
    pub source: String,
    pub current_version: String,
    pub latest_version: String,
}

/// Checks for available updates by comparing local recipe versions with upstream versions
/// Returns a formatted display of packages that need updating
async fn check_updates(root: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    // Scan local recipes
    let recipes = scan_recipes(root)?;

    // Setup progress bar for async operations
    let pb = ProgressBar::new(recipes.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta}) : {msg:.bold}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Process recipes concurrently to check for updates
    let futures = futures::stream::iter(recipes)
        .map(|recipe| {
            let pb = pb.clone();
            async move {
                pb.set_message(recipe.name.to_string());

                // Check if recipe has monitoring info and get latest version
                let latest_version = if let Some(m) = &recipe.monitoring {
                    if m.project_id != 0 {
                        let lv = data::updates::get_latest_version(m.project_id).await?;
                        // Determine next version - prefer stable > latest > first available
                        let next_version = if let Some(stable) = lv.stable_versions.first().cloned()
                        {
                            Some(stable)
                        } else if let Some(latest) = lv.latest_version {
                            Some(latest.clone())
                        } else {
                            lv.versions.first().cloned()
                        };

                        // Create update info if versions differ
                        if let Some(nv) = next_version {
                            if nv != recipe.version {
                                Some(RequiredUpdate {
                                    source: recipe.name.clone(),
                                    current_version: recipe.version.clone(),
                                    latest_version: nv,
                                })
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                pb.inc(1);
                Ok(latest_version) as Result<Option<RequiredUpdate>, Box<dyn std::error::Error>>
            }
        })
        .buffer_unordered(32); // Process up to 32 concurrent requests

    // Collect results
    let latest_recipes: Vec<_> = futures.collect().await;
    pb.finish_and_clear();

    // Filter and sort updates
    let mut updates: Vec<_> = latest_recipes.into_iter().flatten().flatten().collect();
    updates.sort_by(|a, b| a.source.cmp(&b.source));

    // Calculate column widths for pretty printing
    let max_source_len = updates.iter().map(|u| u.source.len()).max().unwrap_or(0);
    let max_current_version_len = updates
        .iter()
        .map(|u| u.current_version.len())
        .max()
        .unwrap_or(0);
    let max_latest_version_len = updates
        .iter()
        .map(|u| u.latest_version.len())
        .max()
        .unwrap_or(0);

    // Print results
    println!(
        "\nTotal packages to update: {}\n",
        updates.len().to_string().yellow()
    );
    // Print header
    println!(
        "{:width_source$} {:width_current$} {:width_latest$}",
        "Package".bold(),
        "Current".bold(),
        "Latest".bold(),
        width_source = max_source_len,
        width_current = max_current_version_len,
        width_latest = max_latest_version_len
    );

    // Print separator line
    println!(
        "{:-<width_source$} {:-<width_current$} {:-<width_latest$}",
        "",
        "",
        "",
        width_source = max_source_len,
        width_current = max_current_version_len,
        width_latest = max_latest_version_len
    );

    // Print updates
    for update in updates {
        println!(
            "{:<width_source$} {:<width_current$} {:<width_latest$}",
            update.source.cyan(),
            update.current_version.red(),
            update.latest_version.green(),
            width_source = max_source_len,
            width_current = max_current_version_len,
            width_latest = max_latest_version_len
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Refresh => {
            todo!("Implement refresh");
        }
        Commands::Check { check_command } => match check_command {
            CheckCommands::Updates => {
                println!("Checking for updates...");
                check_updates(".").await?;
            }
            CheckCommands::Security => {
                todo!("Implement security check");
            }
        },
    }

    Ok(())
}
