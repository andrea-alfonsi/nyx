pub mod runtime;
pub mod compile_time;

// The version of this crate.
// It's important because to register a plugin is necessary to make available a PluginDeclaration with the correct data and layout
pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

// The version of rust that compiled this crate. 
// It's important to ensure that the plugins run the same version because of the missing ABI
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");