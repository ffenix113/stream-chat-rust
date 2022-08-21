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
pub struct ApiError {
    /// Response HTTP status code
    #[serde(rename = "StatusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    /// API error code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Additional error-specific information
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Request duration
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Additional error info
    #[serde(rename = "exception_fields", skip_serializing_if = "Option::is_none")]
    pub exception_fields: Option<::std::collections::HashMap<String, String>>,
    /// Message describing an error
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// URL with additional information
    #[serde(rename = "more_info", skip_serializing_if = "Option::is_none")]
    pub more_info: Option<String>,
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError {
            code: Some(u16::from(err.status().expect("no status code present")) as i32),
            message: Some("request error".to_string()),
            status_code: None,
            details: None,
            duration: None,
            exception_fields: None,
            more_info: None,
        }
    }
}

impl ApiError {
    pub fn new() -> ApiError {
        ApiError {
            status_code: None,
            code: None,
            details: None,
            duration: None,
            exception_fields: None,
            message: None,
            more_info: None,
        }
    }
}
