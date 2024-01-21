/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetLoyaltyStoresCorporationIdOffers200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLoyaltyStoresCorporationIdOffers200Ok {
    /// Analysis kredit cost
    #[serde(rename = "ak_cost", skip_serializing_if = "Option::is_none")]
    pub ak_cost: Option<i32>,
    /// isk_cost integer
    #[serde(rename = "isk_cost")]
    pub isk_cost: i64,
    /// lp_cost integer
    #[serde(rename = "lp_cost")]
    pub lp_cost: i32,
    /// offer_id integer
    #[serde(rename = "offer_id")]
    pub offer_id: i32,
    /// quantity integer
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// required_items array
    #[serde(rename = "required_items")]
    pub required_items: Vec<crate::models::GetLoyaltyStoresCorporationIdOffersRequiredItem>,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetLoyaltyStoresCorporationIdOffers200Ok {
    /// 200 ok object
    pub fn new(
        isk_cost: i64,
        lp_cost: i32,
        offer_id: i32,
        quantity: i32,
        required_items: Vec<crate::models::GetLoyaltyStoresCorporationIdOffersRequiredItem>,
        type_id: i32,
    ) -> GetLoyaltyStoresCorporationIdOffers200Ok {
        GetLoyaltyStoresCorporationIdOffers200Ok {
            ak_cost: None,
            isk_cost,
            lp_cost,
            offer_id,
            quantity,
            required_items,
            type_id,
        }
    }
}
