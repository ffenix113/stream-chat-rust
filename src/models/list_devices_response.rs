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
pub struct ListDevicesResponse {
    /// List of devices
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::Device>>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl ListDevicesResponse {
    pub fn new() -> ListDevicesResponse {
        ListDevicesResponse {
            devices: None,
            duration: None,
        }
    }
}
