/*
 * NiFi Rest API
 *
 * The Rest API provides programmatic access to command and control a NiFi instance in real time. Start and                                             stop processors, monitor queues, query provenance data, and more. Each endpoint below includes a description,                                             definitions of the expected input and output, potential response codes, and the authorizations required                                             to invoke each service.
 *
 * The version of the OpenAPI document: 1.16.0
 * Contact: dev@nifi.apache.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BuildInfo {
    /// The version number of the built component.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The SCM revision id of the source code used for this build.
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// The timestamp (milliseconds since Epoch) of the build.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// The target architecture of the built component.
    #[serde(rename = "targetArch", skip_serializing_if = "Option::is_none")]
    pub target_arch: Option<String>,
    /// The compiler used for the build
    #[serde(rename = "compiler", skip_serializing_if = "Option::is_none")]
    pub compiler: Option<String>,
    /// The compiler flags used for the build.
    #[serde(rename = "compilerFlags", skip_serializing_if = "Option::is_none")]
    pub compiler_flags: Option<String>,
}

impl BuildInfo {
    pub fn new() -> BuildInfo {
        BuildInfo {
            version: None,
            revision: None,
            timestamp: None,
            target_arch: None,
            compiler: None,
            compiler_flags: None,
        }
    }
}


