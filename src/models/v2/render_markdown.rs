////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RenderMarkdownParams {
    pub markdown: String,
}

impl RenderMarkdownParams {
    pub fn new(markdown: String) -> RenderMarkdownParams {
        RenderMarkdownParams { markdown }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RenderMarkdownResponse {
    pub html: String,
}

impl RenderMarkdownResponse {
    pub fn new(html: String) -> RenderMarkdownResponse {
        RenderMarkdownResponse { html }
    }
}
