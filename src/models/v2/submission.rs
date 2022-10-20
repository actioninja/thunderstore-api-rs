////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::models::v2::available_community::AvailableCommunity;
use crate::models::v2::package;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PackageSubmissionMetadata {
    pub author_name: String,
    pub categories: Vec<String>,
    pub communities: Vec<String>,
    pub has_nsfw_content: bool,
    pub upload_uuid: uuid::Uuid,
}

impl PackageSubmissionMetadata {
    #[must_use]
    pub fn new(
        author_name: String,
        categories: Vec<String>,
        communities: Vec<String>,
        has_nsfw_content: bool,
        upload_uuid: uuid::Uuid,
    ) -> PackageSubmissionMetadata {
        PackageSubmissionMetadata {
            author_name,
            categories,
            communities,
            has_nsfw_content,
            upload_uuid,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PackageSubmissionResult {
    pub package_version: package::Version,
    pub available_communities: Vec<AvailableCommunity>,
}

impl PackageSubmissionResult {
    #[must_use]
    pub fn new(
        package_version: package::Version,
        available_communities: Vec<AvailableCommunity>,
    ) -> PackageSubmissionResult {
        PackageSubmissionResult {
            package_version,
            available_communities,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ReadmeValidatorParams {
    pub readme_data: String,
}

impl ReadmeValidatorParams {
    #[must_use]
    pub fn new(readme_data: String) -> ReadmeValidatorParams {
        ReadmeValidatorParams { readme_data }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ValidatorResponse {
    pub success: bool,
}

impl ValidatorResponse {
    #[must_use]
    pub fn new(success: bool) -> ValidatorResponse {
        ValidatorResponse { success }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ManifestV1ValidatorParams {
    pub namespace: String,
    pub manifest_data: String,
}

impl ManifestV1ValidatorParams {
    #[must_use]
    pub fn new(namespace: String, manifest_data: String) -> ManifestV1ValidatorParams {
        ManifestV1ValidatorParams {
            namespace,
            manifest_data,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct IconValidatorParams {
    #[serde(rename = "icon_data")]
    pub icon_data: String,
}

impl IconValidatorParams {
    #[must_use]
    pub fn new(icon_data: String) -> IconValidatorParams {
        IconValidatorParams { icon_data }
    }
}
