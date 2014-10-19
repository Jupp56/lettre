// Copyright 2014 Alexis Mousset. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Rust SMTP library
//!
//! The client does its best to follow RFC 5321 (https://tools.ietf.org/html/rfc5321).
//!
//! It also implements the following extensions :
//!
//! * 8BITMIME (RFC 6152 : https://tools.ietf.org/html/rfc6152)
//! * SIZE (RFC 1427 : https://tools.ietf.org/html/rfc1427)
//!
//! ## What this client is NOT made for
//!
//! Send emails to public email servers. It is not designed to smartly handle servers responses,
//! to rate-limit emails, to make retries, and all that complicated stuff needed to politely
//! talk to public servers.
//!
//! What this client does is basically try once to send the email, and say if it worked.
//! It should only be used to transfer emails to a relay server.
//!
//! ## Usage
//!
//! ```ignore
//! extern crate smtp;
//! use std::io::net::tcp::TcpStream;
//! use smtp::smtpc::client::SmtpClient;
//! use std::string::String;
//!
//! let mut email_client: SmtpClient<TcpStream> =
//!     SmtpClient::new(String::from_str("localhost"), None, None);
//! email_client.send_mail::<TcpStream>(
//!     String::from_str("user@example.com"),
//!     vec!(String::from_str("user@example.org")),
//!     String::from_str("Test email")
//! );
//! ```

#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![desc = "Rust SMTP library"]
#![comment = "Simple SMTP client and library"]
#![license = "MIT/ASL2"]
#![doc(html_root_url = "http://amousset.github.io/rust-smtp/smtp/")]
#![experimental]

#![feature(macro_rules)]
#![feature(phase)]
#![deny(non_camel_case_types)]
#![deny(missing_doc)]
#![deny(unnecessary_qualification)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_typecast)]
#![deny(unused_result)]

#![feature(phase)] #[phase(plugin, link)] extern crate log;

pub mod client;
pub mod command;
pub mod extension;
pub mod response;
pub mod transaction;
pub mod common;