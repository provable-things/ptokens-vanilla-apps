#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;
#[cfg(feature = "json-rpc-db")]
extern crate reqwest;
#[cfg(feature = "json-rpc-db")]
extern crate serde_json;
#[cfg(feature = "file-logger")]
#[cfg(not(test))]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "std-err-logger")]
extern crate stderrlog;
#[cfg(test)]
#[macro_use]
extern crate serial_test;

pub mod databases;
pub mod errors;
pub mod loggers;
pub mod types;
pub mod utils;

pub use ptokens_core::{btc_on_eos, btc_on_eth, eos_on_eth, erc20_on_eos, erc20_on_evm, get_core_version};
