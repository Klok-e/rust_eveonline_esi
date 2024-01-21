/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseStarsStarIdOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUniverseStarsStarIdOk {
    /// Age of star in years
    #[serde(rename = "age")]
    pub age: i64,
    /// luminosity number
    #[serde(rename = "luminosity")]
    pub luminosity: f32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// radius integer
    #[serde(rename = "radius")]
    pub radius: i64,
    /// solar_system_id integer
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
    /// spectral_class string
    #[serde(rename = "spectral_class")]
    pub spectral_class: SpectralClass,
    /// temperature integer
    #[serde(rename = "temperature")]
    pub temperature: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetUniverseStarsStarIdOk {
    /// 200 ok object
    pub fn new(
        age: i64,
        luminosity: f32,
        name: String,
        radius: i64,
        solar_system_id: i32,
        spectral_class: SpectralClass,
        temperature: i32,
        type_id: i32,
    ) -> GetUniverseStarsStarIdOk {
        GetUniverseStarsStarIdOk {
            age,
            luminosity,
            name,
            radius,
            solar_system_id,
            spectral_class,
            temperature,
            type_id,
        }
    }
}

/// spectral_class string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpectralClass {
    #[serde(rename = "K2 V")]
    K2V,
    #[serde(rename = "K4 V")]
    K4V,
    #[serde(rename = "G2 V")]
    G2V,
    #[serde(rename = "G8 V")]
    G8V,
    #[serde(rename = "M7 V")]
    M7V,
    #[serde(rename = "K7 V")]
    K7V,
    #[serde(rename = "M2 V")]
    M2V,
    #[serde(rename = "K5 V")]
    K5V,
    #[serde(rename = "M3 V")]
    M3V,
    #[serde(rename = "G0 V")]
    G0V,
    #[serde(rename = "G7 V")]
    G7V,
    #[serde(rename = "G3 V")]
    G3V,
    #[serde(rename = "F9 V")]
    F9V,
    #[serde(rename = "G5 V")]
    G5V,
    #[serde(rename = "F6 V")]
    F6V,
    #[serde(rename = "K8 V")]
    K8V,
    #[serde(rename = "K9 V")]
    K9V,
    #[serde(rename = "K6 V")]
    K6V,
    #[serde(rename = "G9 V")]
    G9V,
    #[serde(rename = "G6 V")]
    G6V,
    #[serde(rename = "G4 VI")]
    G4Vi,
    #[serde(rename = "G4 V")]
    G4V,
    #[serde(rename = "F8 V")]
    F8V,
    #[serde(rename = "F2 V")]
    F2V,
    #[serde(rename = "F1 V")]
    F1V,
    #[serde(rename = "K3 V")]
    K3V,
    #[serde(rename = "F0 VI")]
    F0Vi,
    #[serde(rename = "G1 VI")]
    G1Vi,
    #[serde(rename = "G0 VI")]
    G0Vi,
    #[serde(rename = "K1 V")]
    K1V,
    #[serde(rename = "M4 V")]
    M4V,
    #[serde(rename = "M1 V")]
    M1V,
    #[serde(rename = "M6 V")]
    M6V,
    #[serde(rename = "M0 V")]
    M0V,
    #[serde(rename = "K2 IV")]
    K2Iv,
    #[serde(rename = "G2 VI")]
    G2Vi,
    #[serde(rename = "K0 V")]
    K0V,
    #[serde(rename = "K5 IV")]
    K5Iv,
    #[serde(rename = "F5 VI")]
    F5Vi,
    #[serde(rename = "G6 VI")]
    G6Vi,
    #[serde(rename = "F6 VI")]
    F6Vi,
    #[serde(rename = "F2 IV")]
    F2Iv,
    #[serde(rename = "G3 VI")]
    G3Vi,
    #[serde(rename = "M8 V")]
    M8V,
    #[serde(rename = "F1 VI")]
    F1Vi,
    #[serde(rename = "K1 IV")]
    K1Iv,
    #[serde(rename = "F7 V")]
    F7V,
    #[serde(rename = "G5 VI")]
    G5Vi,
    #[serde(rename = "M5 V")]
    M5V,
    #[serde(rename = "G7 VI")]
    G7Vi,
    #[serde(rename = "F5 V")]
    F5V,
    #[serde(rename = "F4 VI")]
    F4Vi,
    #[serde(rename = "F8 VI")]
    F8Vi,
    #[serde(rename = "K3 IV")]
    K3Iv,
    #[serde(rename = "F4 IV")]
    F4Iv,
    #[serde(rename = "F0 V")]
    F0V,
    #[serde(rename = "G7 IV")]
    G7Iv,
    #[serde(rename = "G8 VI")]
    G8Vi,
    #[serde(rename = "F2 VI")]
    F2Vi,
    #[serde(rename = "F4 V")]
    F4V,
    #[serde(rename = "F7 VI")]
    F7Vi,
    #[serde(rename = "F3 V")]
    F3V,
    #[serde(rename = "G1 V")]
    G1V,
    #[serde(rename = "G9 VI")]
    G9Vi,
    #[serde(rename = "F3 IV")]
    F3Iv,
    #[serde(rename = "F9 VI")]
    F9Vi,
    #[serde(rename = "M9 V")]
    M9V,
    #[serde(rename = "K0 IV")]
    K0Iv,
    #[serde(rename = "F1 IV")]
    F1Iv,
    #[serde(rename = "G4 IV")]
    G4Iv,
    #[serde(rename = "F3 VI")]
    F3Vi,
    #[serde(rename = "K4 IV")]
    K4Iv,
    #[serde(rename = "G5 IV")]
    G5Iv,
    #[serde(rename = "G3 IV")]
    G3Iv,
    #[serde(rename = "G1 IV")]
    G1Iv,
    #[serde(rename = "K7 IV")]
    K7Iv,
    #[serde(rename = "G0 IV")]
    G0Iv,
    #[serde(rename = "K6 IV")]
    K6Iv,
    #[serde(rename = "K9 IV")]
    K9Iv,
    #[serde(rename = "G2 IV")]
    G2Iv,
    #[serde(rename = "F9 IV")]
    F9Iv,
    #[serde(rename = "F0 IV")]
    F0Iv,
    #[serde(rename = "K8 IV")]
    K8Iv,
    #[serde(rename = "G8 IV")]
    G8Iv,
    #[serde(rename = "F6 IV")]
    F6Iv,
    #[serde(rename = "F5 IV")]
    F5Iv,
    #[serde(rename = "A0")]
    A0,
    #[serde(rename = "A0IV")]
    A0Iv,
    #[serde(rename = "A0IV2")]
    A0Iv2,
}

impl Default for SpectralClass {
    fn default() -> SpectralClass {
        Self::K2V
    }
}
