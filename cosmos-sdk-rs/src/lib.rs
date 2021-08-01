//! # Cosmos SDK for Rust
//!
//! Framework for building [Cosmos] blockchain applications in Rust, modeled off
//! of the [Cosmos SDK for Golang].
//!
//! ## About
//!
//! This library is presently designed to serve as a *client* for interacting
//! with the Golang implementation of the Cosmos SDK.
//!
//! It does not implement server-side functionality (yet), such as hooks
//! and message passing.
//!
//! ## Features
//!
//! - [Transactions][`tx`]: build, sign, and/or parse Cosmos SDK transactions
//!
//! [Cosmos]: https://cosmos.network/
//! [Cosmos SDK for Golang]: https://github.com/cosmos/cosmos-sdk

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png",
    html_root_url = "https://docs.rs/cosmos-sdk/0.1.1"
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod bank;
pub mod distribution;
pub mod crypto;
pub mod staking;
pub mod tx;

#[cfg(feature = "dev")]
#[cfg_attr(docsrs, doc(cfg(feature = "dev")))]
pub mod dev;

mod base;
mod decimal;
mod error;
mod prost_ext;

pub use crate::{
    base::{AccountId, Coin, Denom},
    decimal::Decimal,
    error::{Error, Result},
    tx::Tx,
};

pub use cosmos_sdk_proto as proto;
pub use tendermint;

#[cfg(feature = "rpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "rpc")))]
pub use tendermint_rpc as rpc;
