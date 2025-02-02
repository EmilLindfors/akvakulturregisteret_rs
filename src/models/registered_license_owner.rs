/*
 * Offentlig Akvakulturregister
 *
 * API for det nye offentlige Akvakulturregisteret
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: akva-hjelp@fiskeridir.no
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredLicenseOwner {
    #[serde(rename = "orgNr", skip_serializing_if = "Option::is_none")]
    pub org_nr: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RegisteredLicenseOwner {
    pub fn new() -> RegisteredLicenseOwner {
        RegisteredLicenseOwner {
            org_nr: None,
            name: None,
        }
    }
}
