// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};

/// CVE Data Format specification for Common Vulnerabilities and Exposures (CVE) data
///
/// This module provides types for parsing and representing CVE JSON data according to
/// the official NIST NVD CVE data format
#[derive(Debug, Deserialize, Serialize)]
pub struct CveData {
    #[serde(rename = "CVE_Items")]
    pub cve_items: Vec<CveItem>,
}

/// An individual CVE item containing details about a single vulnerability
#[derive(Debug, Deserialize, Serialize)]
pub struct CveItem {
    pub cve: Cve,
    pub configurations: Configurations,
    pub impact: Impact,
    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: String,
    #[serde(rename = "publishedDate")]
    pub published_date: String,
}

/// Core CVE data including identifier, description and references
#[derive(Debug, Deserialize, Serialize)]
pub struct Cve {
    #[serde(rename = "CVE_data_meta")]
    pub data_meta: CveDataMeta,
    pub description: Description,
    pub references: References,
}

/// CVE metadata containing the unique identifier
#[derive(Debug, Deserialize, Serialize)]
pub struct CveDataMeta {
    #[serde(rename = "ID")]
    pub id: String,
}

/// Description of the vulnerability
#[derive(Debug, Deserialize, Serialize)]
pub struct Description {
    #[serde(rename = "description_data")]
    pub data: Vec<DescriptionData>,
}

/// Individual description text with language tag
#[derive(Debug, Deserialize, Serialize)]
pub struct DescriptionData {
    pub lang: String,
    pub value: String,
}

/// References to external sources about the vulnerability
#[derive(Debug, Deserialize, Serialize)]
pub struct References {
    #[serde(rename = "reference_data")]
    pub data: Vec<ReferenceData>,
}

/// Individual reference with URL and metadata
#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceData {
    pub url: String,
    pub name: Option<String>,
    #[serde(rename = "refsource")]
    pub ref_source: Option<String>,
}

/// Configuration information describing affected products
#[derive(Debug, Deserialize, Serialize)]
pub struct Configurations {
    #[serde(rename = "CVE_data_version")]
    pub data_version: String,
    pub nodes: Vec<Node>,
}

/// Node in the configuration tree
#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub operator: String,
    pub children: Option<Vec<Node>>,
    #[serde(rename = "cpe_match")]
    pub cpe_match: Option<Vec<CpeMatch>>,
}

/// CPE match rules describing affected versions
#[derive(Debug, Deserialize, Serialize)]
pub struct CpeMatch {
    pub vulnerable: bool,
    #[serde(rename = "cpe23Uri")]
    pub cpe23_uri: String,
    #[serde(rename = "versionStartIncluding")]
    pub version_start_including: Option<String>,
    #[serde(rename = "versionEndIncluding")]
    pub version_end_including: Option<String>,
    #[serde(rename = "versionStartExcluding")]
    pub version_start_excluding: Option<String>,
    #[serde(rename = "versionEndExcluding")]
    pub version_end_excluding: Option<String>,
}

/// Impact scores and metrics for the vulnerability
#[derive(Debug, Deserialize, Serialize)]
pub struct Impact {
    #[serde(rename = "baseMetricV3")]
    pub base_metric_v3: Option<BaseMetricV3>,
    #[serde(rename = "baseMetricV2")]
    pub base_metric_v2: Option<BaseMetricV2>,
}

/// CVSS v3 base metrics
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseMetricV3 {
    #[serde(rename = "cvssV3")]
    pub cvss_v3: CvssV3,
    #[serde(rename = "exploitabilityScore")]
    pub exploitability_score: f64,
    #[serde(rename = "impactScore")]
    pub impact_score: f64,
}

/// CVSS v3 scoring vector details
#[derive(Debug, Deserialize, Serialize)]
pub struct CvssV3 {
    #[serde(rename = "vectorString")]
    pub vector_string: String,
    #[serde(rename = "attackVector")]
    pub attack_vector: String,
    #[serde(rename = "attackComplexity")]
    pub attack_complexity: String,
    #[serde(rename = "privilegesRequired")]
    pub privileges_required: String,
    #[serde(rename = "userInteraction")]
    pub user_interaction: String,
    pub scope: String,
    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: String,
    #[serde(rename = "integrityImpact")]
    pub integrity_impact: String,
    #[serde(rename = "availabilityImpact")]
    pub availability_impact: String,
    #[serde(rename = "baseScore")]
    pub base_score: f64,
    #[serde(rename = "baseSeverity")]
    pub base_severity: String,
}

/// CVSS v2 base metrics
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseMetricV2 {
    #[serde(rename = "cvssV2")]
    pub cvss_v2: CvssV2,
    #[serde(rename = "exploitabilityScore")]
    pub exploitability_score: f64,
    #[serde(rename = "impactScore")]
    pub impact_score: f64,
}

/// CVSS v2 scoring details
#[derive(Debug, Deserialize, Serialize)]
pub struct CvssV2 {
    pub version: String,
    #[serde(rename = "vectorString")]
    pub vector_string: String,
    #[serde(rename = "baseScore")]
    pub base_score: f64,
}
