////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Package {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_downloads: Option<String>,
    pub latest: Box<Version>,
    pub community_listings: Vec<Listing>,
}

impl Package {
    pub fn new(name: String, latest: Version, community_listings: Vec<Listing>) -> Package {
        Package {
            namespace: None,
            name,
            full_name: None,
            owner: None,
            package_url: None,
            date_created: None,
            date_updated: None,
            rating_score: None,
            is_pinned: None,
            is_deprecated: None,
            total_downloads: None,
            latest: Box::new(latest),
            community_listings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Card {
    pub categories: Vec<Category>,
    pub community_identifier: String,
    pub community_name: String,
    pub description: String,
    pub download_count: u32,
    pub image_src: Option<String>,
    pub is_deprecated: bool,
    pub is_nsfw: bool,
    pub is_pinned: bool,
    pub last_updated: String,
    pub namespace: String,
    pub package_name: String,
    pub rating_score: u32,
    pub team_name: String,
}

impl Card {
    pub fn new(
        categories: Vec<Category>,
        community_identifier: String,
        community_name: String,
        description: String,
        download_count: u32,
        image_src: Option<String>,
        is_deprecated: bool,
        is_nsfw: bool,
        is_pinned: bool,
        last_updated: String,
        namespace: String,
        package_name: String,
        rating_score: u32,
        team_name: String,
    ) -> Card {
        Card {
            categories,
            community_identifier,
            community_name,
            description,
            download_count,
            image_src,
            is_deprecated,
            is_nsfw,
            is_pinned,
            last_updated,
            namespace,
            package_name,
            rating_score,
            team_name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub slug: String,
}

impl Category {
    pub fn new(name: String, slug: String) -> Category {
        Category { name, slug }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dependency {
    pub community_identifier: Option<String>,
    pub community_name: Option<String>,
    pub description: String,
    pub image_src: Option<String>,
    pub namespace: String,
    pub package_name: String,
    pub version_number: String,
}

impl Dependency {
    pub fn new(
        community_identifier: Option<String>,
        community_name: Option<String>,
        description: String,
        image_src: Option<String>,
        namespace: String,
        package_name: String,
        version_number: String,
    ) -> Dependency {
        Dependency {
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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Version {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub name: String,
    pub version_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    pub website_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl Version {
    pub fn new(
        name: String,
        version_number: String,
        description: String,
        website_url: String,
    ) -> Version {
        Version {
            namespace: None,
            name,
            version_number,
            full_name: None,
            description,
            icon: None,
            dependencies: None,
            download_url: None,
            downloads: None,
            date_created: None,
            website_url,
            is_active: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Listing {
    #[serde(rename = "has_nsfw_content", skip_serializing_if = "Option::is_none")]
    pub has_nsfw_content: Option<bool>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    #[serde(rename = "community", skip_serializing_if = "Option::is_none")]
    pub community: Option<String>,
    #[serde(rename = "review_status", skip_serializing_if = "Option::is_none")]
    pub review_status: Option<ReviewStatus>,
}

impl Listing {
    pub fn new() -> Listing {
        Listing {
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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetailView {
    pub bg_image_src: Option<String>,
    pub categories: Vec<Category>,
    pub community_identifier: String,
    pub community_name: String,
    pub dependant_count: u32,
    pub dependencies: Vec<Dependency>,
    pub dependency_string: String,
    pub description: String,
    pub download_count: u32,
    pub download_url: String,
    pub image_src: Option<String>,
    pub install_url: String,
    pub last_updated: String,
    pub markdown: String,
    pub namespace: String,
    pub package_name: String,
    pub rating_score: u32,
    pub team_name: String,
    pub versions: Vec<Version>,
    pub website: String,
}

impl DetailView {
    pub fn new(
        bg_image_src: Option<String>,
        categories: Vec<Category>,
        community_identifier: String,
        community_name: String,
        dependant_count: u32,
        dependencies: Vec<Dependency>,
        dependency_string: String,
        description: String,
        download_count: u32,
        download_url: String,
        image_src: Option<String>,
        install_url: String,
        last_updated: String,
        markdown: String,
        namespace: String,
        package_name: String,
        rating_score: u32,
        team_name: String,
        versions: Vec<Version>,
        website: String,
    ) -> DetailView {
        DetailView {
            bg_image_src,
            categories,
            community_identifier,
            community_name,
            dependant_count,
            dependencies,
            dependency_string,
            description,
            download_count,
            download_url,
            image_src,
            install_url,
            last_updated,
            markdown,
            namespace,
            package_name,
            rating_score,
            team_name,
            versions,
            website,
        }
    }
}
