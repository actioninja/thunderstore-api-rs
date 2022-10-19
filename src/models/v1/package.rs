////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Listing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donation_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_nsfw_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<String>,
}

impl Listing {
    pub fn new() -> Listing {
        Listing {
            name: None,
            full_name: None,
            owner: None,
            package_url: None,
            donation_link: None,
            date_created: None,
            date_updated: None,
            uuid4: None,
            rating_score: None,
            is_pinned: None,
            is_deprecated: None,
            has_nsfw_content: None,
            categories: None,
            versions: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PackageVersion {
    pub date_created: String,
    pub download_count: u32,
    pub download_url: String,
    pub install_url: String,
    pub version_number: String,
}

impl PackageVersion {
    pub fn new(
        date_created: String,
        download_count: u32,
        download_url: String,
        install_url: String,
        version_number: String,
    ) -> PackageVersion {
        PackageVersion {
            date_created,
            download_count,
            download_url,
            install_url,
            version_number,
        }
    }
}
