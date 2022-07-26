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
pub struct RegistryClientsEntity {
    #[serde(rename = "registries", skip_serializing_if = "Option::is_none")]
    pub registries: Option<Vec<crate::models::RegistryClientEntity>>,
}

impl RegistryClientsEntity {
    pub fn new() -> RegistryClientsEntity {
        RegistryClientsEntity {
            registries: None,
        }
    }
}


