////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "ts-rs")]
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct RenderMarkdownParams {
    pub markdown: String,
}

impl RenderMarkdownParams {
    #[must_use]
    pub fn new(markdown: String) -> RenderMarkdownParams {
        RenderMarkdownParams { markdown }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct RenderMarkdownResponse {
    pub html: String,
}

impl RenderMarkdownResponse {
    #[must_use]
    pub fn new(html: String) -> RenderMarkdownResponse {
        RenderMarkdownResponse { html }
    }
}
