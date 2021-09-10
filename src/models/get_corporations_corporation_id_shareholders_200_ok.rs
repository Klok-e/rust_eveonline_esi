/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdShareholders200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdShareholders200Ok {
    /// share_count integer
    #[serde(rename = "share_count")]
    pub share_count: i64,
    /// shareholder_id integer
    #[serde(rename = "shareholder_id")]
    pub shareholder_id: i32,
    /// shareholder_type string
    #[serde(rename = "shareholder_type")]
    pub shareholder_type: ShareholderType,
}

impl GetCorporationsCorporationIdShareholders200Ok {
    /// 200 ok object
    pub fn new(share_count: i64, shareholder_id: i32, shareholder_type: ShareholderType) -> GetCorporationsCorporationIdShareholders200Ok {
        GetCorporationsCorporationIdShareholders200Ok {
            share_count,
            shareholder_id,
            shareholder_type,
        }
    }
}

/// shareholder_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShareholderType {
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "corporation")]
    Corporation,
}
