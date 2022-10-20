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

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RequestBody {
    pub code: String,
    pub redirect_uri: String,
}

impl RequestBody {
    #[must_use] pub fn new(code: String, redirect_uri: String) -> RequestBody {
        RequestBody { code, redirect_uri }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ResponseBody {
    pub email: Option<String>,
    pub session_id: String,
    pub username: String,
}

impl ResponseBody {
    #[must_use] pub fn new(email: Option<String>, session_id: String, username: String) -> ResponseBody {
        ResponseBody {
            email,
            session_id,
            username,
        }
    }
}
