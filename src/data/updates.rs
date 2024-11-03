// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

/// Version response from release-monitoring.org
#[derive(Debug, Deserialize)]
pub struct VersionResponse {
    pub latest_version: Option<String>,

    #[serde(default)]
    pub stable_versions: Vec<String>,

    #[serde(default)]
    pub versions: Vec<String>,
}

/// This method will return the latest version of a project
/// from release-monitoring.org API using the project_id
pub async fn get_latest_version(project_id: i64) -> Result<VersionResponse, reqwest::Error> {
    let url = format!(
        "https://release-monitoring.org/api/v2/versions/?project_id={}",
        project_id
    );
    let response = reqwest::get(&url).await?.json().await?;
    Ok(response)
}
