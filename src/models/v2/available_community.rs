////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::models::Community;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AvailableCommunity {
    pub community: Community,
    pub categories: Vec<crate::models::Category>,
    pub url: String,
}

impl AvailableCommunity {
    pub fn new(
        community: Community,
        categories: Vec<crate::models::Category>,
        url: String,
    ) -> AvailableCommunity {
        AvailableCommunity {
            community,
            categories,
            url,
        }
    }
}
