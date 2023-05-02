use std::{collections::HashMap, rc::Rc};
use libloading::Library;
use super::FunctionProxy;

/// This is the default plugin registrar. Actually is not public because there are no alternatives
/// It's used to load all the functions before pushing them in the `FunctionsRegistry`
pub(crate) struct PluginRegistrar<Prototype: ?Sized>{
  pub functions: HashMap< String, FunctionProxy<Prototype>>,
  lib: Rc<Library>
}

impl<Prototype: ?Sized> crate::runtime::plugin::PluginRegistrar<Prototype> for PluginRegistrar<Prototype> {
    fn register_function(&mut self, name: &str, function: Box<Prototype>) {
        let proxy = FunctionProxy { function, _lib: Rc::clone( &self.lib )};
        self.functions.insert( name.to_string(), proxy);
    }
}

impl<Prototype: ?Sized> PluginRegistrar<Prototype>{
  pub fn new( lib: Rc<Library>) -> Self {
    PluginRegistrar::<Prototype> { functions: HashMap::new(), lib }
  }
}