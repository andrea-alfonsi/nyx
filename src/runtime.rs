pub mod app;
pub mod plugin;

pub use app::*;
pub use plugin::*;

use std::alloc::System;

// Currently the default global allocator is unspecified. Libraries, however, 
// like cdylibs and staticlibs are guaranteed to use the System by default.
#[global_allocator]
static ALLOCATOR: System = System;