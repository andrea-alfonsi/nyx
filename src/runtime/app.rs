pub mod functions_registry;
pub mod registrar;

pub use functions_registry::*;
pub(crate) use registrar::*;

use std::{rc::Rc, ops::Deref};
use libloading::Library;

/// This is a wrapper around the plugin. This automatically dereference to the 
/// inner `Prototype` simulating he implementation of the prototype.
pub struct FunctionProxy<Prototype: ?Sized> {
  function: Box<Prototype>,
  _lib: Rc<Library>
}

impl<Prototype: ?Sized> Deref for FunctionProxy<Prototype>{
  type Target = Box<Prototype>;
  fn deref(&self) -> &Self::Target {
    &self.function
  }
}

