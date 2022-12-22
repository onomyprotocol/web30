#![warn(clippy::all)]
#![allow(clippy::pedantic)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub mod amm;
pub mod client;
mod erc20_utils;
pub mod eth_wrapping;
mod event_utils;
pub mod jsonrpc;
mod mem;
pub mod types;

pub use event_utils::address_to_event;

#[cfg(feature = "record_json_rpc")]
lazy_static! {
    pub static ref JSON_RPC_COUNTER: std::sync::Mutex<u64> = std::sync::Mutex::new(0);
    pub static ref JSON_RPC_REQUESTS: std::sync::Mutex<Vec<(u64, String)>> =
        std::sync::Mutex::new(vec![]);
    pub static ref JSON_RPC_RESPONSES: std::sync::Mutex<Vec<(u64, String)>> =
        std::sync::Mutex::new(vec![]);
}
