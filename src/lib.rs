////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#![warn(clippy::pedantic, clippy::cargo)]
// Default::default() is more idiomatic imo
#![allow(clippy::default_trait_access)]
// too many lines is a dumb metric
#![allow(clippy::too_many_lines)]
// as is fine, clippy is silly
#![allow(clippy::cast_lossless)]
// Not actually going to be a published crate, useless to add
#![allow(clippy::cargo_common_metadata)]
// Annoying
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate serde_derive;

pub mod apis;
pub mod models;
