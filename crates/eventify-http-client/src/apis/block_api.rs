/*
 * eventify-http-server
 *
 * Ethereum Indexer
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: lachezarkolevgg@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`get_blocks_count`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlocksCountError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// Get the Count of Blocks  This endpoint returns the total count of blocks present in the database. The response is a JSON object containing the count.  # Responses  * `200 OK`: Successfully retrieved the count of blocks. The response body will be a JSON object with the structure `{ \"count\": i64 }`, where `i64` is the total number of blocks. * `500 Internal Server Error`: Indicates that an error occurred on the server while processing the request. The response body will contain a JSON object with an error message.  # Example  ```json { \"count\": 42 } ```
pub async fn get_blocks_count(
    configuration: &configuration::Configuration,
) -> Result<(), Error<GetBlocksCountError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/blocks/count", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetBlocksCountError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
