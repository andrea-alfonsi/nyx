// The version of this crate.
// It's important because to register a plugin is necessary to make available a PluginDeclaration with the correct data and layout
pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

// The version of rust that compiled this crate. 
// It's important to ensure that the plugins run the same version because of the missing ABI
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// A system makes the design more simple and clear leaving all the complex part of extracting the right arguments at the compiler.
pub mod system;

/// A collection of tools and utilities to test plugins before releasing.
pub mod pdk;

/// Exports all the requirements to make a plugin compatible with the host.
/// Please see the `Limitations` section inoerder to understan what isor isnot possible to do and how to solve some problems.
pub mod plugin;

/// Constains allthe traits that the plugin system should support. 
pub mod host;