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

/// LicenseCapacityHistory : Kapasitetshistorikken til tillatelsen.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseCapacityHistory {
    /// Den akkumulerte kapasiteten som tillatelsen har oppnådd.
    #[serde(rename = "accumulatedCapacity", skip_serializing_if = "Option::is_none")]
    pub accumulated_capacity: Option<f64>,
    /// Den nåværende kapasiteten til tillatelsen.
    #[serde(rename = "currentCapacity", skip_serializing_if = "Option::is_none")]
    pub current_capacity: Option<f64>,
    /// Enhetstypen til kapasiteten f.eks. Tonn.
    #[serde(rename = "capacityUnitType", skip_serializing_if = "Option::is_none")]
    pub capacity_unit_type: Option<String>,
    /// Verditypen til kapasiteten.
    #[serde(rename = "capacityValueType", skip_serializing_if = "Option::is_none")]
    pub capacity_value_type: Option<String>,
    /// Kapasitetstypen
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Typeverdi.
    #[serde(rename = "typeValue", skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Verdi
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// Verditype
    #[serde(rename = "valueType", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    /// Verdien til typeverdien.
    #[serde(rename = "valueTypeValue", skip_serializing_if = "Option::is_none")]
    pub value_type_value: Option<String>,
    /// Tidsstempel for når tillatelsen ble gyldig fra.
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    /// Tidsstempel for når tillatelsen er gyldig til.
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    /// Status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Statusverdi
    #[serde(rename = "statusValue", skip_serializing_if = "Option::is_none")]
    pub status_value: Option<String>,
    #[serde(rename = "atLeastFrom", skip_serializing_if = "Option::is_none")]
    pub at_least_from: Option<String>,
    #[serde(rename = "atLeastUntil", skip_serializing_if = "Option::is_none")]
    pub at_least_until: Option<String>,
}

impl LicenseCapacityHistory {
    /// Kapasitetshistorikken til tillatelsen.
    pub fn new() -> LicenseCapacityHistory {
        LicenseCapacityHistory {
            accumulated_capacity: None,
            current_capacity: None,
            capacity_unit_type: None,
            capacity_value_type: None,
            r#type: None,
            type_value: None,
            value: None,
            value_type: None,
            value_type_value: None,
            valid_from: None,
            valid_until: None,
            status: None,
            status_value: None,
            at_least_from: None,
            at_least_until: None,
        }
    }
}

