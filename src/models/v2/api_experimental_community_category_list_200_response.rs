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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiExperimentalCommunityCategoryList200Response {
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::Category>,
}

impl ApiExperimentalCommunityCategoryList200Response {
    pub fn new(
        results: Vec<crate::models::Category>,
    ) -> ApiExperimentalCommunityCategoryList200Response {
        ApiExperimentalCommunityCategoryList200Response {
            next: None,
            previous: None,
            results,
        }
    }
}
