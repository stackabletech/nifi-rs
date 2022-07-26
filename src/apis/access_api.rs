/*
 * NiFi Rest API
 *
 * The Rest API provides programmatic access to command and control a NiFi instance in real time. Start and                                             stop processors, monitor queues, query provenance data, and more. Each endpoint below includes a description,                                             definitions of the expected input and output, potential response codes, and the authorizations required                                             to invoke each service.
 *
 * The version of the OpenAPI document: 1.16.0
 * Contact: dev@nifi.apache.org
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_access_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAccessTokenError {
    Status400(),
    Status403(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_access_token_from_ticket`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAccessTokenFromTicketError {
    Status400(),
    Status401(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_access_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccessStatusError {
    Status400(),
    Status401(),
    Status403(),
    Status409(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_login_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoginConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`knox_callback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KnoxCallbackError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`knox_logout`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KnoxLogoutError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`knox_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KnoxRequestError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`log_out`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogOutError {
    Status401(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`log_out_complete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogOutCompleteError {
    Status401(),
    Status500(),
    UnknownValue(serde_json::Value),
}


/// The token returned is formatted as a JSON Web Token (JWT). The token is base64 encoded and comprised of three parts. The header, the body, and the signature. The expiration of the token is a contained within the body. It is stored in the browser as a cookie, but also returned inthe response body to be stored/used by third party client scripts.
pub async fn create_access_token(configuration: &configuration::Configuration, username: Option<&str>, password: Option<&str>) -> Result<String, Error<CreateAccessTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/token", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = username {
        local_var_form_params.insert("username", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("password", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(local_var_content.to_string())
    } else {
        let local_var_entity: Option<CreateAccessTokenError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The token returned is formatted as a JSON Web Token (JWT). The token is base64 encoded and comprised of three parts. The header, the body, and the signature. The expiration of the token is a contained within the body. The token can be used in the Authorization header in the format 'Authorization: Bearer <token>'. It is also stored in the browser as a cookie.
pub async fn create_access_token_from_ticket(configuration: &configuration::Configuration, ) -> Result<String, Error<CreateAccessTokenFromTicketError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/kerberos", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAccessTokenFromTicketError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
pub async fn get_access_status(configuration: &configuration::Configuration, ) -> Result<crate::models::AccessStatusEntity, Error<GetAccessStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAccessStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_login_config(configuration: &configuration::Configuration, ) -> Result<crate::models::AccessConfigurationEntity, Error<GetLoginConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/config", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLoginConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
pub async fn knox_callback(configuration: &configuration::Configuration, ) -> Result<(), Error<KnoxCallbackError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/knox/callback", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<KnoxCallbackError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
pub async fn knox_logout(configuration: &configuration::Configuration, ) -> Result<(), Error<KnoxLogoutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/knox/logout", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<KnoxLogoutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
pub async fn knox_request(configuration: &configuration::Configuration, ) -> Result<(), Error<KnoxRequestError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/knox/request", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<KnoxRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
pub async fn log_out(configuration: &configuration::Configuration, ) -> Result<(), Error<LogOutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/logout", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LogOutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
pub async fn log_out_complete(configuration: &configuration::Configuration, ) -> Result<(), Error<LogOutCompleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/access/logout/complete", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_configuration.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LogOutCompleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

