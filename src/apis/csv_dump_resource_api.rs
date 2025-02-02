/*
 * Offentlig Akvakulturregister
 *
 * API for det nye offentlige Akvakulturregisteret
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: akva-hjelp@fiskeridir.no
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
use csv::{ReaderBuilder, StringRecord};
#[cfg(feature = "mockall")]
use mockall::automock;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

#[cfg_attr(feature = "mockall", automock)]
#[async_trait]
pub trait CsvDumpResourceApi: Send + Sync {
    async fn get_new_legacy_csv_dump(
        &self,
    ) -> Result<ResponseContent<GetNewLegacyCsvDumpSuccess>, Error<GetNewLegacyCsvDumpError>>;

    // Change return type to handle binary data
    async fn get_new_legacy_csv_file_dump(
        &self,
    ) -> Result<Vec<u8>, Error<GetNewLegacyCsvFileDumpError>>;
}

pub struct CsvDumpResourceApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl CsvDumpResourceApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl CsvDumpResourceApi for CsvDumpResourceApiClient {
    /// deprecated
    async fn get_new_legacy_csv_dump(
        &self,
    ) -> Result<ResponseContent<GetNewLegacyCsvDumpSuccess>, Error<GetNewLegacyCsvDumpError>> {
        let local_var_configuration = &self.configuration;
        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/dump/new-legacy-csv",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            // Skip the first line and remove any empty lines
            let content_lines: Vec<&str> = local_var_content
                .lines()
                .skip(1) // Skip the title row
                .filter(|line| !line.trim().is_empty())
                .collect();

            // Create a new string with the filtered content
            let filtered_content = content_lines.join("\n");

            // Configure CSV reader with appropriate settings
            let mut csv_reader = ReaderBuilder::new()
                .delimiter(b';')
                .flexible(true) // Handle variable number of fields
                .trim(csv::Trim::All) // Trim whitespace
                .from_reader(filtered_content.as_bytes());

            let headers: Vec<String> = csv_reader
                .headers()
                .unwrap_or(&StringRecord::new())
                .iter()
                .map(|header| header.to_string())
                .collect();

            let records: Result<Vec<Value>, _> = csv_reader
                .records()
                .map(|record| {
                    let record = record?;

                    let obj: Value = json!(headers
                        .iter()
                        .zip(record.iter())
                        .map(|(header, value)| (header.clone(), Value::String(value.to_string())))
                        .collect::<serde_json::Map<String, Value>>());
                    Ok::<Value, csv::Error>(obj)
                })
                .collect();

            let json_array = Value::Array(records.map_err(|e| {
                Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    content: "csv parsing error".to_string(),
                    entity: Some(GetNewLegacyCsvDumpError::UnknownValue(
                        json!({"error": e.to_string()}),
                    )),
                })
            })?);

            let local_var_entity = Some(GetNewLegacyCsvDumpSuccess::Status200(json_array));
            let local_var_result = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Ok(local_var_result)
        } else {
            let local_var_entity: Option<GetNewLegacyCsvDumpError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_new_legacy_csv_file_dump(
        &self,
    ) -> Result<Vec<u8>, Error<GetNewLegacyCsvFileDumpError>> {
        let local_var_configuration = &self.configuration;
        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/dump/new-legacy-csv-file",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(local_var_resp.bytes().await?.to_vec())
        } else {
            let local_var_content = local_var_resp.text().await?;
            let local_var_entity: Option<GetNewLegacyCsvFileDumpError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed successes of method [`get_new_legacy_csv_dump`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewLegacyCsvDumpSuccess {
    Status200(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_new_legacy_csv_file_dump`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewLegacyCsvFileDumpSuccess {
    Status200(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_new_legacy_csv_dump`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewLegacyCsvDumpError {
    Status400(std::collections::HashMap<String, Vec<String>>),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_new_legacy_csv_file_dump`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewLegacyCsvFileDumpError {
    Status400(std::collections::HashMap<String, Vec<String>>),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
