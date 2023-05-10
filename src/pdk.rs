//! This is the Package Developement Kit. Abbreviated as PDK.
//! This is just an utility that plugin developer may use to test their plugins before releasing them.
//! It contains some implementations that may not respect the final ones, but they should give many 
//! informations about the plugin behavior and the correctness of the code
//! 
//! ## Example
//! This code suppose you have both `lib.rs` and `bin.rs` files. In the first one you have all the structs required by the plugin to work properly,
//! while the second one will import the PDK and runthe tests
//! 
//! `lib.rs`
//! ```
//! struct MyPlugin { /* ...  */ }
//! // Logic to export the plugin here
//! 
//! ```
//! 
//! `bin.rs`
//! ```
//! use aanyx::pdk::*;
//! // Logic to test the plugin here
//! ```