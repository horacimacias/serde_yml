// Copyright 2016 Serde YAML Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))] // turn warnings into errors

extern crate dtoa;
extern crate linked_hash_map;
extern crate serde;
extern crate yaml_rust;

pub use self::de::{Deserializer, from_iter, from_reader, from_slice, from_str};
pub use self::ser::{Serializer, to_string, to_vec, to_writer};
pub use self::value::{Value, from_value, to_value};
pub use self::error::{Error, Result};

#[macro_use]
mod forward;

pub mod de;
pub mod ser;
pub mod value;
pub mod error;
