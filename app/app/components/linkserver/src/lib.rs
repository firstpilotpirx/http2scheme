//! Link server component: opens allowed custom URL scheme links from keyed loopback requests.

#![forbid(unsafe_code)]

pub mod adapters;
pub mod core;
pub mod linkserver_config;
pub mod linkserver_ports;
pub mod linkserver_setup;
pub mod open_link_ports;
