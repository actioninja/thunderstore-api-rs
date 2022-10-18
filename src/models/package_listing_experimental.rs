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
pub struct PackageListingExperimental {
    #[serde(rename = "has_nsfw_content", skip_serializing_if = "Option::is_none")]
    pub has_nsfw_content: Option<bool>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    #[serde(rename = "community", skip_serializing_if = "Option::is_none")]
    pub community: Option<String>,
    #[serde(rename = "review_status", skip_serializing_if = "Option::is_none")]
    pub review_status: Option<ReviewStatus>,
}

impl PackageListingExperimental {
    pub fn new() -> PackageListingExperimental {
        PackageListingExperimental {
            has_nsfw_content: None,
            categories: None,
            community: None,
            review_status: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReviewStatus {
    #[serde(rename = "unreviewed")]
    Unreviewed,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for ReviewStatus {
    fn default() -> ReviewStatus {
        Self::Unreviewed
    }
}

