/*
 * Thunderstore API
 *
 * Schema is automatically generated and not completely accurate.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiExperimentalCommunityCategoryList200Response {
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::PackageCategory>,
}

impl ApiExperimentalCommunityCategoryList200Response {
    pub fn new(results: Vec<crate::models::PackageCategory>) -> ApiExperimentalCommunityCategoryList200Response {
        ApiExperimentalCommunityCategoryList200Response {
            next: None,
            previous: None,
            results,
        }
    }
}

