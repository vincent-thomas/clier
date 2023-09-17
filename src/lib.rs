//! # Command Line Argument Parser for Rust
//! `Clier` is a command line argument parser and command framework for rust.
//!
//! ## Parser
//! To start a new cli projects run:
//!
//! ```console
#![doc = include_str!("../doc/cargo_add.md")]
//! ```
//!
//! Then define your CLI in `main.rs`:
//!
//! ```rust
#![doc = include_str!("../examples/parser.rs")]
//! ```
//!
//! And try it out:
//! ```md
#![doc = include_str!("../doc/demo_output.md")]
//! ```
//!
//! ## Framework
//! soon...

mod cli;
pub use cli::*;
pub mod command;
pub mod error;
pub mod help;
pub mod hooks;
mod prelude;
pub mod run;

mod format;
mod utils;
