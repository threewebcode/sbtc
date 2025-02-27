/*
 * emily-openapi-spec
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// HealthData : Struct that represents the current status of the API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthData {
    /// The version of the API.
    #[serde(rename = "version")]
    pub version: String,
}

impl HealthData {
    /// Struct that represents the current status of the API.
    pub fn new(version: String) -> HealthData {
        HealthData { version }
    }
}
