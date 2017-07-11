// Copyright (C) 2017 1aim GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

extern crate regex;
extern crate regex_cache;
extern crate fnv;
extern crate quick_xml as xml;
extern crate itertools;
extern crate either;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

#[macro_use]
mod helper;

/// Errors for various parts of the crate.
pub mod error;
pub use error::{Error, ErrorKind, Result};

/// Phone number metadata, containing patterns, formatting and other useful
/// data about countries and phone numbers.
pub mod metadata;
pub use metadata::Metadata;

mod consts;

mod national_number;
pub use national_number::NationalNumber;

pub mod country;
pub use country::Country;

mod extension;
pub use extension::Extension;

mod carrier;
pub use carrier::Carrier;

mod phone_number;
pub use phone_number::{PhoneNumber, Type};

mod parser;
pub use parser::{parse, parse_with};

pub mod formatter;
pub use formatter::{format, format_with};

mod validator;
pub use validator::{Validation, is_viable, is_valid, is_valid_with};
