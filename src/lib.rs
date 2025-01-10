#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;
/// # Akvakulturregisteret API Client
///
/// API for det nye offentlige Akvakulturregisteret
///
/// Example
/// ```rust
/// use akvareg::apis::area_resource_api::{AreaResourceApiClient, AreaResourceApi};
/// use akvareg::configuration::Configuration;
/// use std::sync::Arc;
///
/// #[tokio::main]
/// async fn main() {
///     let configuration = Configuration::new();
///     let api_client = AreaResourceApiClient::new(Arc::new(configuration));
///     let version_id = "1";
///     let response = api_client.area(version_id).await;
///     match response {
///         Ok(response) => {
///             println!("{:?}", response);
///         }
///         Err(error) => {
///             println!("{:?}", error);
///         }
///     }
/// }
/// ```
pub mod apis;
pub mod models;
