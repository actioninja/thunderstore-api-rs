////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::models::v2::community;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FrontPageContent {
    #[serde(rename = "communities")]
    pub communities: Vec<community::Card>,
    #[serde(rename = "download_count")]
    pub download_count: u32,
    #[serde(rename = "package_count")]
    pub package_count: u32,
}

impl FrontPageContent {
    pub fn new(
        communities: Vec<community::Card>,
        download_count: u32,
        package_count: u32,
    ) -> FrontPageContent {
        FrontPageContent {
            communities,
            download_count,
            package_count,
        }
    }
}
