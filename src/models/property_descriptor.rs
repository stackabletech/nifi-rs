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
pub struct PropertyDescriptor {
    /// The name of the property key
    #[serde(rename = "name")]
    pub name: String,
    /// The display name of the property key, if different from the name
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The description of what the property does
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of the allowable values for the property
    #[serde(rename = "allowableValues", skip_serializing_if = "Option::is_none")]
    pub allowable_values: Option<Vec<crate::models::PropertyAllowableValue>>,
    /// The default value if a user-set value is not specified
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Whether or not  the property is required for the component
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Whether or not  the value of the property is considered sensitive (e.g., passwords and keys)
    #[serde(rename = "sensitive", skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// The scope of expression language supported by this property
    #[serde(rename = "expressionLanguageScope", skip_serializing_if = "Option::is_none")]
    pub expression_language_scope: Option<ExpressionLanguageScope>,
    /// The description of the expression language scope supported by this property
    #[serde(rename = "expressionLanguageScopeDescription", skip_serializing_if = "Option::is_none")]
    pub expression_language_scope_description: Option<String>,
    #[serde(rename = "typeProvidedByValue", skip_serializing_if = "Option::is_none")]
    pub type_provided_by_value: Option<Box<crate::models::DefinedType>>,
    /// A regular expression that can be used to validate the value of this property
    #[serde(rename = "validRegex", skip_serializing_if = "Option::is_none")]
    pub valid_regex: Option<String>,
    /// Name of the validator used for this property descriptor
    #[serde(rename = "validator", skip_serializing_if = "Option::is_none")]
    pub validator: Option<String>,
    /// Whether or not the descriptor is for a dynamically added property
    #[serde(rename = "dynamic", skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    #[serde(rename = "resourceDefinition", skip_serializing_if = "Option::is_none")]
    pub resource_definition: Option<Box<crate::models::PropertyResourceDefinition>>,
    /// The dependencies that this property has on other properties
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::models::PropertyDependency>>,
}

impl PropertyDescriptor {
    pub fn new(name: String) -> PropertyDescriptor {
        PropertyDescriptor {
            name,
            display_name: None,
            description: None,
            allowable_values: None,
            default_value: None,
            required: None,
            sensitive: None,
            expression_language_scope: None,
            expression_language_scope_description: None,
            type_provided_by_value: None,
            valid_regex: None,
            validator: None,
            dynamic: None,
            resource_definition: None,
            dependencies: None,
        }
    }
}

/// The scope of expression language supported by this property
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExpressionLanguageScope {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "VARIABLE_REGISTRY")]
    VARIABLEREGISTRY,
    #[serde(rename = "FLOWFILE_ATTRIBUTES")]
    FLOWFILEATTRIBUTES,
}

impl Default for ExpressionLanguageScope {
    fn default() -> ExpressionLanguageScope {
        Self::NONE
    }
}

