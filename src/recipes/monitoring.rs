// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0
use serde::Deserialize;
use thiserror::Error;

/// CPE ID
#[derive(Debug, Deserialize)]
pub struct CpeID {
    pub vendor: String,
    pub product: String,
}

/// Monitoring data
/// This struct represents the data found in a monitoring YAML file
///
/// # Example
///
/// ```yaml
/// releases:
///  id: 1234
/// security:
/// cpe:
/// - vendor: "vendor"
///  product: "product"
/// ```
///
#[derive(Debug)]
pub struct Monitoring {
    /// Project ID on release-monitoring.org
    pub project_id: i64,

    /// Found in the `security -> cpe` part of monitorng YAML
    pub cpes: Vec<CpeID>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing monitoring YAML")]
    ParseError(#[from] serde_yaml::Error),
}

#[derive(serde::Deserialize)]
struct MonitoringYAML {
    pub releases: Option<ReleasesYAML>,
    pub security: Option<SecurityYAML>,
}

// This is the struct that represents the releases part of the monitoring YAML
#[derive(serde::Deserialize)]
struct ReleasesYAML {
    pub id: Option<i64>,
}

// This is the struct that represents the security part of the monitoring YAML
#[derive(serde::Deserialize)]
struct SecurityYAML {
    pub cpe: Option<Vec<CpeID>>,
}

impl Monitoring {
    /// Parse a monitoring YAML string
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let m: MonitoringYAML = serde_yaml::from_str(s).map_err(Error::ParseError)?;

        let project_id = m.releases.and_then(|r| r.id).unwrap_or(0);
        let cpes = m.security.and_then(|s| s.cpe).unwrap_or_default();

        Ok(Monitoring { project_id, cpes })
    }
}
