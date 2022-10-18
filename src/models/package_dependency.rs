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
pub struct PackageDependency {
    #[serde(rename = "community_identifier")]
    pub community_identifier: Option<String>,
    #[serde(rename = "community_name")]
    pub community_name: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "image_src")]
    pub image_src: Option<String>,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "package_name")]
    pub package_name: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
}

impl PackageDependency {
    pub fn new(community_identifier: Option<String>, community_name: Option<String>, description: String, image_src: Option<String>, namespace: String, package_name: String, version_number: String) -> PackageDependency {
        PackageDependency {
            community_identifier,
            community_name,
            description,
            image_src,
            namespace,
            package_name,
            version_number,
        }
    }
}

