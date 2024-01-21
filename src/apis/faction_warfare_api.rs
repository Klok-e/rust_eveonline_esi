/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`get_characters_character_id_fw_stats`]
#[derive(Clone, Debug)]
pub struct GetCharactersCharacterIdFwStatsParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`get_corporations_corporation_id_fw_stats`]
#[derive(Clone, Debug)]
pub struct GetCorporationsCorporationIdFwStatsParams {
    /// An EVE corporation ID
    pub corporation_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`get_fw_leaderboards`]
#[derive(Clone, Debug)]
pub struct GetFwLeaderboardsParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_fw_leaderboards_characters`]
#[derive(Clone, Debug)]
pub struct GetFwLeaderboardsCharactersParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_fw_leaderboards_corporations`]
#[derive(Clone, Debug)]
pub struct GetFwLeaderboardsCorporationsParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_fw_stats`]
#[derive(Clone, Debug)]
pub struct GetFwStatsParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_fw_systems`]
#[derive(Clone, Debug)]
pub struct GetFwSystemsParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_fw_wars`]
#[derive(Clone, Debug)]
pub struct GetFwWarsParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for typed successes of method [`get_characters_character_id_fw_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdFwStatsSuccess {
    Status200(crate::models::GetCharactersCharacterIdFwStatsOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_corporations_corporation_id_fw_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdFwStatsSuccess {
    Status200(crate::models::GetCorporationsCorporationIdFwStatsOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_fw_leaderboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwLeaderboardsSuccess {
    Status200(crate::models::GetFwLeaderboardsOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_fw_leaderboards_characters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwLeaderboardsCharactersSuccess {
    Status200(crate::models::GetFwLeaderboardsCharactersOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_fw_leaderboards_corporations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwLeaderboardsCorporationsSuccess {
    Status200(crate::models::GetFwLeaderboardsCorporationsOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_fw_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwStatsSuccess {
    Status200(Vec<crate::models::GetFwStats200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_fw_systems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwSystemsSuccess {
    Status200(Vec<crate::models::GetFwSystems200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_fw_wars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwWarsSuccess {
    Status200(Vec<crate::models::GetFwWars200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_fw_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdFwStatsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_corporations_corporation_id_fw_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdFwStatsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_fw_leaderboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwLeaderboardsError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_fw_leaderboards_characters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwLeaderboardsCharactersError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_fw_leaderboards_corporations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwLeaderboardsCorporationsError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_fw_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwStatsError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_fw_systems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwSystemsError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_fw_wars`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFwWarsError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// Statistical overview of a character involved in faction warfare  --- Alternate route: `/dev/characters/{character_id}/fw/stats/`  Alternate route: `/legacy/characters/{character_id}/fw/stats/`  Alternate route: `/v1/characters/{character_id}/fw/stats/`  Alternate route: `/v2/characters/{character_id}/fw/stats/`  --- This route expires daily at 11:05
pub async fn get_characters_character_id_fw_stats(
    configuration: &configuration::Configuration,
    params: GetCharactersCharacterIdFwStatsParams,
) -> Result<
    ResponseContent<GetCharactersCharacterIdFwStatsSuccess>,
    Error<GetCharactersCharacterIdFwStatsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/characters/{character_id}/fw/stats/",
        local_var_configuration.base_path,
        character_id = character_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdFwStatsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdFwStatsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Statistics about a corporation involved in faction warfare  --- Alternate route: `/dev/corporations/{corporation_id}/fw/stats/`  Alternate route: `/legacy/corporations/{corporation_id}/fw/stats/`  Alternate route: `/v1/corporations/{corporation_id}/fw/stats/`  Alternate route: `/v2/corporations/{corporation_id}/fw/stats/`  --- This route expires daily at 11:05
pub async fn get_corporations_corporation_id_fw_stats(
    configuration: &configuration::Configuration,
    params: GetCorporationsCorporationIdFwStatsParams,
) -> Result<
    ResponseContent<GetCorporationsCorporationIdFwStatsSuccess>,
    Error<GetCorporationsCorporationIdFwStatsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let corporation_id = params.corporation_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/corporations/{corporation_id}/fw/stats/",
        local_var_configuration.base_path,
        corporation_id = corporation_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCorporationsCorporationIdFwStatsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCorporationsCorporationIdFwStatsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Top 4 leaderboard of factions for kills and victory points separated by total, last week and yesterday  --- Alternate route: `/dev/fw/leaderboards/`  Alternate route: `/legacy/fw/leaderboards/`  Alternate route: `/v1/fw/leaderboards/`  Alternate route: `/v2/fw/leaderboards/`  --- This route expires daily at 11:05
pub async fn get_fw_leaderboards(
    configuration: &configuration::Configuration,
    params: GetFwLeaderboardsParams,
) -> Result<ResponseContent<GetFwLeaderboardsSuccess>, Error<GetFwLeaderboardsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fw/leaderboards/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetFwLeaderboardsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetFwLeaderboardsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Top 100 leaderboard of pilots for kills and victory points separated by total, last week and yesterday  --- Alternate route: `/dev/fw/leaderboards/characters/`  Alternate route: `/legacy/fw/leaderboards/characters/`  Alternate route: `/v1/fw/leaderboards/characters/`  Alternate route: `/v2/fw/leaderboards/characters/`  --- This route expires daily at 11:05
pub async fn get_fw_leaderboards_characters(
    configuration: &configuration::Configuration,
    params: GetFwLeaderboardsCharactersParams,
) -> Result<
    ResponseContent<GetFwLeaderboardsCharactersSuccess>,
    Error<GetFwLeaderboardsCharactersError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/fw/leaderboards/characters/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetFwLeaderboardsCharactersSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetFwLeaderboardsCharactersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Top 10 leaderboard of corporations for kills and victory points separated by total, last week and yesterday  --- Alternate route: `/dev/fw/leaderboards/corporations/`  Alternate route: `/legacy/fw/leaderboards/corporations/`  Alternate route: `/v1/fw/leaderboards/corporations/`  Alternate route: `/v2/fw/leaderboards/corporations/`  --- This route expires daily at 11:05
pub async fn get_fw_leaderboards_corporations(
    configuration: &configuration::Configuration,
    params: GetFwLeaderboardsCorporationsParams,
) -> Result<
    ResponseContent<GetFwLeaderboardsCorporationsSuccess>,
    Error<GetFwLeaderboardsCorporationsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/fw/leaderboards/corporations/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetFwLeaderboardsCorporationsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetFwLeaderboardsCorporationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Statistical overviews of factions involved in faction warfare  --- Alternate route: `/dev/fw/stats/`  Alternate route: `/legacy/fw/stats/`  Alternate route: `/v1/fw/stats/`  Alternate route: `/v2/fw/stats/`  --- This route expires daily at 11:05
pub async fn get_fw_stats(
    configuration: &configuration::Configuration,
    params: GetFwStatsParams,
) -> Result<ResponseContent<GetFwStatsSuccess>, Error<GetFwStatsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fw/stats/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetFwStatsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetFwStatsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// An overview of the current ownership of faction warfare solar systems  --- Alternate route: `/dev/fw/systems/`  Alternate route: `/legacy/fw/systems/`  Alternate route: `/v2/fw/systems/`  Alternate route: `/v3/fw/systems/`  --- This route is cached for up to 1800 seconds
pub async fn get_fw_systems(
    configuration: &configuration::Configuration,
    params: GetFwSystemsParams,
) -> Result<ResponseContent<GetFwSystemsSuccess>, Error<GetFwSystemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fw/systems/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetFwSystemsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetFwSystemsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Data about which NPC factions are at war  --- Alternate route: `/dev/fw/wars/`  Alternate route: `/legacy/fw/wars/`  Alternate route: `/v1/fw/wars/`  Alternate route: `/v2/fw/wars/`  --- This route expires daily at 11:05
pub async fn get_fw_wars(
    configuration: &configuration::Configuration,
    params: GetFwWarsParams,
) -> Result<ResponseContent<GetFwWarsSuccess>, Error<GetFwWarsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fw/wars/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetFwWarsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetFwWarsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
