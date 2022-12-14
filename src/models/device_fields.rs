/*
 * Stream Chat API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v71.1.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeviceFields {
    /// Device ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "push_provider", skip_serializing_if = "Option::is_none")]
    pub push_provider: Option<PushProvider>,
    /// Name of the push provider configuration
    #[serde(rename = "push_provider_name", skip_serializing_if = "Option::is_none")]
    pub push_provider_name: Option<String>,
}

impl DeviceFields {
    pub fn new() -> DeviceFields {
        DeviceFields {
            id: None,
            push_provider: None,
            push_provider_name: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PushProvider {
    #[serde(rename = "firebase")]
    Firebase,
    #[serde(rename = "apn")]
    Apn,
    #[serde(rename = "huawei")]
    Huawei,
    #[serde(rename = "xiaomi")]
    Xiaomi,
}

impl Default for PushProvider {
    fn default() -> PushProvider {
        Self::Firebase
    }
}

