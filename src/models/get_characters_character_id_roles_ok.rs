/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdRolesOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdRolesOk {
    /// roles array
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Roles>>,
    /// roles_at_base array
    #[serde(rename = "roles_at_base", skip_serializing_if = "Option::is_none")]
    pub roles_at_base: Option<Vec<RolesAtBase>>,
    /// roles_at_hq array
    #[serde(rename = "roles_at_hq", skip_serializing_if = "Option::is_none")]
    pub roles_at_hq: Option<Vec<RolesAtHq>>,
    /// roles_at_other array
    #[serde(rename = "roles_at_other", skip_serializing_if = "Option::is_none")]
    pub roles_at_other: Option<Vec<RolesAtOther>>,
}

impl GetCharactersCharacterIdRolesOk {
    /// 200 ok object
    pub fn new() -> GetCharactersCharacterIdRolesOk {
        GetCharactersCharacterIdRolesOk {
            roles: None,
            roles_at_base: None,
            roles_at_hq: None,
            roles_at_other: None,
        }
    }
}

/// roles array
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Roles {
    #[serde(rename = "Account_Take_1")]
    AccountTake1,
    #[serde(rename = "Account_Take_2")]
    AccountTake2,
    #[serde(rename = "Account_Take_3")]
    AccountTake3,
    #[serde(rename = "Account_Take_4")]
    AccountTake4,
    #[serde(rename = "Account_Take_5")]
    AccountTake5,
    #[serde(rename = "Account_Take_6")]
    AccountTake6,
    #[serde(rename = "Account_Take_7")]
    AccountTake7,
    #[serde(rename = "Accountant")]
    Accountant,
    #[serde(rename = "Auditor")]
    Auditor,
    #[serde(rename = "Communications_Officer")]
    CommunicationsOfficer,
    #[serde(rename = "Config_Equipment")]
    ConfigEquipment,
    #[serde(rename = "Config_Starbase_Equipment")]
    ConfigStarbaseEquipment,
    #[serde(rename = "Container_Take_1")]
    ContainerTake1,
    #[serde(rename = "Container_Take_2")]
    ContainerTake2,
    #[serde(rename = "Container_Take_3")]
    ContainerTake3,
    #[serde(rename = "Container_Take_4")]
    ContainerTake4,
    #[serde(rename = "Container_Take_5")]
    ContainerTake5,
    #[serde(rename = "Container_Take_6")]
    ContainerTake6,
    #[serde(rename = "Container_Take_7")]
    ContainerTake7,
    #[serde(rename = "Contract_Manager")]
    ContractManager,
    #[serde(rename = "Diplomat")]
    Diplomat,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Factory_Manager")]
    FactoryManager,
    #[serde(rename = "Fitting_Manager")]
    FittingManager,
    #[serde(rename = "Hangar_Query_1")]
    HangarQuery1,
    #[serde(rename = "Hangar_Query_2")]
    HangarQuery2,
    #[serde(rename = "Hangar_Query_3")]
    HangarQuery3,
    #[serde(rename = "Hangar_Query_4")]
    HangarQuery4,
    #[serde(rename = "Hangar_Query_5")]
    HangarQuery5,
    #[serde(rename = "Hangar_Query_6")]
    HangarQuery6,
    #[serde(rename = "Hangar_Query_7")]
    HangarQuery7,
    #[serde(rename = "Hangar_Take_1")]
    HangarTake1,
    #[serde(rename = "Hangar_Take_2")]
    HangarTake2,
    #[serde(rename = "Hangar_Take_3")]
    HangarTake3,
    #[serde(rename = "Hangar_Take_4")]
    HangarTake4,
    #[serde(rename = "Hangar_Take_5")]
    HangarTake5,
    #[serde(rename = "Hangar_Take_6")]
    HangarTake6,
    #[serde(rename = "Hangar_Take_7")]
    HangarTake7,
    #[serde(rename = "Junior_Accountant")]
    JuniorAccountant,
    #[serde(rename = "Personnel_Manager")]
    PersonnelManager,
    #[serde(rename = "Rent_Factory_Facility")]
    RentFactoryFacility,
    #[serde(rename = "Rent_Office")]
    RentOffice,
    #[serde(rename = "Rent_Research_Facility")]
    RentResearchFacility,
    #[serde(rename = "Security_Officer")]
    SecurityOfficer,
    #[serde(rename = "Starbase_Defense_Operator")]
    StarbaseDefenseOperator,
    #[serde(rename = "Starbase_Fuel_Technician")]
    StarbaseFuelTechnician,
    #[serde(rename = "Station_Manager")]
    StationManager,
    #[serde(rename = "Trader")]
    Trader,
}
/// roles_at_base array
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RolesAtBase {
    #[serde(rename = "Account_Take_1")]
    AccountTake1,
    #[serde(rename = "Account_Take_2")]
    AccountTake2,
    #[serde(rename = "Account_Take_3")]
    AccountTake3,
    #[serde(rename = "Account_Take_4")]
    AccountTake4,
    #[serde(rename = "Account_Take_5")]
    AccountTake5,
    #[serde(rename = "Account_Take_6")]
    AccountTake6,
    #[serde(rename = "Account_Take_7")]
    AccountTake7,
    #[serde(rename = "Accountant")]
    Accountant,
    #[serde(rename = "Auditor")]
    Auditor,
    #[serde(rename = "Communications_Officer")]
    CommunicationsOfficer,
    #[serde(rename = "Config_Equipment")]
    ConfigEquipment,
    #[serde(rename = "Config_Starbase_Equipment")]
    ConfigStarbaseEquipment,
    #[serde(rename = "Container_Take_1")]
    ContainerTake1,
    #[serde(rename = "Container_Take_2")]
    ContainerTake2,
    #[serde(rename = "Container_Take_3")]
    ContainerTake3,
    #[serde(rename = "Container_Take_4")]
    ContainerTake4,
    #[serde(rename = "Container_Take_5")]
    ContainerTake5,
    #[serde(rename = "Container_Take_6")]
    ContainerTake6,
    #[serde(rename = "Container_Take_7")]
    ContainerTake7,
    #[serde(rename = "Contract_Manager")]
    ContractManager,
    #[serde(rename = "Diplomat")]
    Diplomat,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Factory_Manager")]
    FactoryManager,
    #[serde(rename = "Fitting_Manager")]
    FittingManager,
    #[serde(rename = "Hangar_Query_1")]
    HangarQuery1,
    #[serde(rename = "Hangar_Query_2")]
    HangarQuery2,
    #[serde(rename = "Hangar_Query_3")]
    HangarQuery3,
    #[serde(rename = "Hangar_Query_4")]
    HangarQuery4,
    #[serde(rename = "Hangar_Query_5")]
    HangarQuery5,
    #[serde(rename = "Hangar_Query_6")]
    HangarQuery6,
    #[serde(rename = "Hangar_Query_7")]
    HangarQuery7,
    #[serde(rename = "Hangar_Take_1")]
    HangarTake1,
    #[serde(rename = "Hangar_Take_2")]
    HangarTake2,
    #[serde(rename = "Hangar_Take_3")]
    HangarTake3,
    #[serde(rename = "Hangar_Take_4")]
    HangarTake4,
    #[serde(rename = "Hangar_Take_5")]
    HangarTake5,
    #[serde(rename = "Hangar_Take_6")]
    HangarTake6,
    #[serde(rename = "Hangar_Take_7")]
    HangarTake7,
    #[serde(rename = "Junior_Accountant")]
    JuniorAccountant,
    #[serde(rename = "Personnel_Manager")]
    PersonnelManager,
    #[serde(rename = "Rent_Factory_Facility")]
    RentFactoryFacility,
    #[serde(rename = "Rent_Office")]
    RentOffice,
    #[serde(rename = "Rent_Research_Facility")]
    RentResearchFacility,
    #[serde(rename = "Security_Officer")]
    SecurityOfficer,
    #[serde(rename = "Starbase_Defense_Operator")]
    StarbaseDefenseOperator,
    #[serde(rename = "Starbase_Fuel_Technician")]
    StarbaseFuelTechnician,
    #[serde(rename = "Station_Manager")]
    StationManager,
    #[serde(rename = "Trader")]
    Trader,
}
/// roles_at_hq array
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RolesAtHq {
    #[serde(rename = "Account_Take_1")]
    AccountTake1,
    #[serde(rename = "Account_Take_2")]
    AccountTake2,
    #[serde(rename = "Account_Take_3")]
    AccountTake3,
    #[serde(rename = "Account_Take_4")]
    AccountTake4,
    #[serde(rename = "Account_Take_5")]
    AccountTake5,
    #[serde(rename = "Account_Take_6")]
    AccountTake6,
    #[serde(rename = "Account_Take_7")]
    AccountTake7,
    #[serde(rename = "Accountant")]
    Accountant,
    #[serde(rename = "Auditor")]
    Auditor,
    #[serde(rename = "Communications_Officer")]
    CommunicationsOfficer,
    #[serde(rename = "Config_Equipment")]
    ConfigEquipment,
    #[serde(rename = "Config_Starbase_Equipment")]
    ConfigStarbaseEquipment,
    #[serde(rename = "Container_Take_1")]
    ContainerTake1,
    #[serde(rename = "Container_Take_2")]
    ContainerTake2,
    #[serde(rename = "Container_Take_3")]
    ContainerTake3,
    #[serde(rename = "Container_Take_4")]
    ContainerTake4,
    #[serde(rename = "Container_Take_5")]
    ContainerTake5,
    #[serde(rename = "Container_Take_6")]
    ContainerTake6,
    #[serde(rename = "Container_Take_7")]
    ContainerTake7,
    #[serde(rename = "Contract_Manager")]
    ContractManager,
    #[serde(rename = "Diplomat")]
    Diplomat,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Factory_Manager")]
    FactoryManager,
    #[serde(rename = "Fitting_Manager")]
    FittingManager,
    #[serde(rename = "Hangar_Query_1")]
    HangarQuery1,
    #[serde(rename = "Hangar_Query_2")]
    HangarQuery2,
    #[serde(rename = "Hangar_Query_3")]
    HangarQuery3,
    #[serde(rename = "Hangar_Query_4")]
    HangarQuery4,
    #[serde(rename = "Hangar_Query_5")]
    HangarQuery5,
    #[serde(rename = "Hangar_Query_6")]
    HangarQuery6,
    #[serde(rename = "Hangar_Query_7")]
    HangarQuery7,
    #[serde(rename = "Hangar_Take_1")]
    HangarTake1,
    #[serde(rename = "Hangar_Take_2")]
    HangarTake2,
    #[serde(rename = "Hangar_Take_3")]
    HangarTake3,
    #[serde(rename = "Hangar_Take_4")]
    HangarTake4,
    #[serde(rename = "Hangar_Take_5")]
    HangarTake5,
    #[serde(rename = "Hangar_Take_6")]
    HangarTake6,
    #[serde(rename = "Hangar_Take_7")]
    HangarTake7,
    #[serde(rename = "Junior_Accountant")]
    JuniorAccountant,
    #[serde(rename = "Personnel_Manager")]
    PersonnelManager,
    #[serde(rename = "Rent_Factory_Facility")]
    RentFactoryFacility,
    #[serde(rename = "Rent_Office")]
    RentOffice,
    #[serde(rename = "Rent_Research_Facility")]
    RentResearchFacility,
    #[serde(rename = "Security_Officer")]
    SecurityOfficer,
    #[serde(rename = "Starbase_Defense_Operator")]
    StarbaseDefenseOperator,
    #[serde(rename = "Starbase_Fuel_Technician")]
    StarbaseFuelTechnician,
    #[serde(rename = "Station_Manager")]
    StationManager,
    #[serde(rename = "Trader")]
    Trader,
}
/// roles_at_other array
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RolesAtOther {
    #[serde(rename = "Account_Take_1")]
    AccountTake1,
    #[serde(rename = "Account_Take_2")]
    AccountTake2,
    #[serde(rename = "Account_Take_3")]
    AccountTake3,
    #[serde(rename = "Account_Take_4")]
    AccountTake4,
    #[serde(rename = "Account_Take_5")]
    AccountTake5,
    #[serde(rename = "Account_Take_6")]
    AccountTake6,
    #[serde(rename = "Account_Take_7")]
    AccountTake7,
    #[serde(rename = "Accountant")]
    Accountant,
    #[serde(rename = "Auditor")]
    Auditor,
    #[serde(rename = "Communications_Officer")]
    CommunicationsOfficer,
    #[serde(rename = "Config_Equipment")]
    ConfigEquipment,
    #[serde(rename = "Config_Starbase_Equipment")]
    ConfigStarbaseEquipment,
    #[serde(rename = "Container_Take_1")]
    ContainerTake1,
    #[serde(rename = "Container_Take_2")]
    ContainerTake2,
    #[serde(rename = "Container_Take_3")]
    ContainerTake3,
    #[serde(rename = "Container_Take_4")]
    ContainerTake4,
    #[serde(rename = "Container_Take_5")]
    ContainerTake5,
    #[serde(rename = "Container_Take_6")]
    ContainerTake6,
    #[serde(rename = "Container_Take_7")]
    ContainerTake7,
    #[serde(rename = "Contract_Manager")]
    ContractManager,
    #[serde(rename = "Diplomat")]
    Diplomat,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Factory_Manager")]
    FactoryManager,
    #[serde(rename = "Fitting_Manager")]
    FittingManager,
    #[serde(rename = "Hangar_Query_1")]
    HangarQuery1,
    #[serde(rename = "Hangar_Query_2")]
    HangarQuery2,
    #[serde(rename = "Hangar_Query_3")]
    HangarQuery3,
    #[serde(rename = "Hangar_Query_4")]
    HangarQuery4,
    #[serde(rename = "Hangar_Query_5")]
    HangarQuery5,
    #[serde(rename = "Hangar_Query_6")]
    HangarQuery6,
    #[serde(rename = "Hangar_Query_7")]
    HangarQuery7,
    #[serde(rename = "Hangar_Take_1")]
    HangarTake1,
    #[serde(rename = "Hangar_Take_2")]
    HangarTake2,
    #[serde(rename = "Hangar_Take_3")]
    HangarTake3,
    #[serde(rename = "Hangar_Take_4")]
    HangarTake4,
    #[serde(rename = "Hangar_Take_5")]
    HangarTake5,
    #[serde(rename = "Hangar_Take_6")]
    HangarTake6,
    #[serde(rename = "Hangar_Take_7")]
    HangarTake7,
    #[serde(rename = "Junior_Accountant")]
    JuniorAccountant,
    #[serde(rename = "Personnel_Manager")]
    PersonnelManager,
    #[serde(rename = "Rent_Factory_Facility")]
    RentFactoryFacility,
    #[serde(rename = "Rent_Office")]
    RentOffice,
    #[serde(rename = "Rent_Research_Facility")]
    RentResearchFacility,
    #[serde(rename = "Security_Officer")]
    SecurityOfficer,
    #[serde(rename = "Starbase_Defense_Operator")]
    StarbaseDefenseOperator,
    #[serde(rename = "Starbase_Fuel_Technician")]
    StarbaseFuelTechnician,
    #[serde(rename = "Station_Manager")]
    StationManager,
    #[serde(rename = "Trader")]
    Trader,
}
