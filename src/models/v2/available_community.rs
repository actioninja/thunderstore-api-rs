////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::models::v2::community::Community;
use crate::models::v2::package::Category;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AvailableCommunity {
    pub community: Community,
    pub categories: Vec<Category>,
    pub url: String,
}

impl AvailableCommunity {
    #[must_use] pub fn new(community: Community, categories: Vec<Category>, url: String) -> AvailableCommunity {
        AvailableCommunity {
            community,
            categories,
            url,
        }
    }
}
