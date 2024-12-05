// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(from = "i32")]
pub enum BuildStatus {
    New = 0,
    Failed = 1,
    Building = 2,
    Publishing = 3,
    Completed = 4,
    Blocked = 5,
}

impl From<i32> for BuildStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => BuildStatus::New,
            1 => BuildStatus::Failed,
            2 => BuildStatus::Building,
            3 => BuildStatus::Publishing,
            4 => BuildStatus::Completed,
            5 => BuildStatus::Blocked,
            _ => BuildStatus::Failed, // Default to Failed for unknown values
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct TaskEnumerateResponse {
    pub items: Vec<Task>,
    #[serde(rename = "numPages")]
    pub num_pages: i32,
    pub page: i32,
    #[serde(rename = "hasPrevious")]
    pub has_previous: bool,
    #[serde(rename = "hasNext")]
    pub has_next: bool,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: i64,
    #[serde(rename = "projectID")]
    pub project_id: i64,
    #[serde(rename = "repoID")]
    pub repo_id: i64,
    #[serde(rename = "profileID")]
    pub profile_id: i64,
    pub slug: String,
    #[serde(rename = "pkgID")]
    pub pkg_id: String,
    pub architecture: String,
    #[serde(rename = "buildID")]
    pub build_id: String,
    pub description: String,
    #[serde(rename = "commitRef")]
    pub commit_ref: String,
    #[serde(rename = "sourcePath")]
    pub source_path: String,
    pub status: BuildStatus,
    #[serde(rename = "tsStarted")]
    pub ts_started: i64,
    #[serde(rename = "tsUpdated")]
    pub ts_updated: i64,
    #[serde(rename = "tsEnded")]
    pub ts_ended: i64,
    #[serde(rename = "blockedBy", default)]
    pub blocked_by: Vec<String>,
    #[serde(rename = "allocatedBuilder")]
    pub allocated_builder: String,
    #[serde(rename = "logPath")]
    pub log_path: String,
}
