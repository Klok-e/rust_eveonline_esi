/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetContractsPublicBidsContractIdNotFound : Not found

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetContractsPublicBidsContractIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetContractsPublicBidsContractIdNotFound {
    /// Not found
    pub fn new() -> GetContractsPublicBidsContractIdNotFound {
        GetContractsPublicBidsContractIdNotFound { error: None }
    }
}
