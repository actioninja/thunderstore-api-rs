////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::models::v2::package;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Community {
    pub identifier: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_package_listing_approval: Option<bool>,
}

impl Community {
    #[must_use] pub fn new(identifier: String, name: String) -> Community {
        Community {
            identifier,
            name,
            discord_url: None,
            wiki_url: None,
            require_package_listing_approval: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PackageList {
    pub bg_image_src: Option<String>,
    pub categories: Vec<package::Category>,
    pub community_name: String,
    pub has_more_pages: bool,
    pub packages: Vec<package::Card>,
}

impl PackageList {
    #[must_use] pub fn new(
        bg_image_src: Option<String>,
        categories: Vec<package::Category>,
        community_name: String,
        has_more_pages: bool,
        packages: Vec<package::Card>,
    ) -> PackageList {
        PackageList {
            bg_image_src,
            categories,
            community_name,
            has_more_pages,
            packages,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Card {
    pub bg_image_src: Option<String>,
    pub download_count: u32,
    pub identifier: String,
    pub name: String,
    pub package_count: u32,
}

impl Card {
    #[must_use] pub fn new(
        bg_image_src: Option<String>,
        download_count: u32,
        identifier: String,
        name: String,
        package_count: u32,
    ) -> Card {
        Card {
            bg_image_src,
            download_count,
            identifier,
            name,
            package_count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<Community>,
}

impl ListResponse {
    #[must_use] pub fn new(results: Vec<Community>) -> ListResponse {
        ListResponse {
            next: None,
            previous: None,
            results,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CategoryListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<package::Category>,
}

impl CategoryListResponse {
    #[must_use] pub fn new(results: Vec<package::Category>) -> CategoryListResponse {
        CategoryListResponse {
            next: None,
            previous: None,
            results,
        }
    }
}
