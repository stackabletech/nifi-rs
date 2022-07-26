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
pub struct RuntimeManifest {
    /// A unique identifier for the manifest
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The type of the runtime binary, e.g., 'minifi-java' or 'minifi-cpp'
    #[serde(rename = "agentType", skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    /// The version of the runtime binary, e.g., '1.0.1'
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "buildInfo", skip_serializing_if = "Option::is_none")]
    pub build_info: Option<Box<crate::models::BuildInfo>>,
    /// All extension bundles included with this runtime
    #[serde(rename = "bundles", skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<crate::models::Bundle>>,
    #[serde(rename = "schedulingDefaults", skip_serializing_if = "Option::is_none")]
    pub scheduling_defaults: Option<Box<crate::models::SchedulingDefaults>>,
}

impl RuntimeManifest {
    pub fn new() -> RuntimeManifest {
        RuntimeManifest {
            identifier: None,
            agent_type: None,
            version: None,
            build_info: None,
            bundles: None,
            scheduling_defaults: None,
        }
    }
}


