use std::{collections::{HashMap, hash_map::Keys}, ffi::OsStr, rc::Rc};
use libloading::Library;
use crate::runtime::{PluginDeclaration, app::PluginRegistrar, FunctionProxy};

/// This a wrapper around a HashMap that ensures all the libraries loaded will live long enough.
pub struct FunctionsRegistry<Prototype: ?Sized> {
  functions: HashMap< String, FunctionProxy<Prototype> >,
  libraries: Vec<Rc<Library>>
}

impl<Prototype: ?Sized> Default for FunctionsRegistry<Prototype>{
  fn default() -> Self {
    Self { functions: Default::default(), libraries: Vec::new() }
  }
}

impl<Prototype: ?Sized> FunctionsRegistry<Prototype> {
  /// ```
  /// use aanyx::runtime::FunctionsRegistry;
  /// # trait Prototype {};
  /// 
  /// let mut registry = FunctionsRegistry::<dyn Prototype>::new();
  /// ```
  /// ```
  /// use aanyx::runtime::FunctionsRegistry;
  /// # trait Prototype {};
  /// 
  /// // Not registry can no longer load new plugins
  /// let registry = unsafe {
  ///   let mut tmp_registry = FunctionsRegistry::<dyn Prototype>::new();
  ///   tmp_registry.load( "tmp/to/plugin" );
  ///   tmp_registry.load( "tmp/to/another/plugin" );
  /// };
  /// ```
  pub fn new() -> Self { FunctionsRegistry::<Prototype>::default() }

  /// Load a new plugin from a library and register all the functions inside
  /// ```
  /// use aanyx::runtime::FunctionsRegistry;
  /// 
  /// trait Call {
  ///   fn call( &self );
  /// }
  /// 
  /// let mut registry = FunctionsRegistry::<dyn Call>::new();
  /// let load_result = unsafe { registry.load( "path/to/my_plugin" ) };
  /// # assert!( load_result.is_err() )
  /// ```
  pub unsafe fn load<P: AsRef<OsStr>>( &mut self, path: P) -> std::io::Result<()> {
    let library = Rc::new( match Library::new(path) {
      Ok(l) => l,
      Err(e) => {
        return Err( std::io::Error::new( std::io::ErrorKind::Other, e));
      }
    });
    let decl = match library.get::<*mut PluginDeclaration>(b"plugin_declaration\0"){
      Ok(s) => s.read(),
      Err(e)=>{
        return Err( std::io::Error::new( std::io::ErrorKind::Other, e)); 
      }
     };
    if decl.rustc_version != crate::RUSTC_VERSION || decl.nyx_version != crate::CORE_VERSION {
      return Err(std::io::Error::new(
          std::io::ErrorKind::Other,
          "Version mismatch",
      ));
    }
    let mut registrar = PluginRegistrar::new(Rc::clone(&library));
    (decl.register)(&mut registrar);
    // Force type coercion. This is really bad to see but it works.
    // It may produce lots of Undefinite Behaviors as described here: https://doc.rust-lang.org/std/mem/fn.transmute.html
    self.functions.extend(
      std::mem::transmute::<
        HashMap<String, FunctionProxy<dyn std::any::Any>>, 
        HashMap<String, FunctionProxy<Prototype>> >(registrar.functions));

    self.libraries.push(library);

    Ok(())
  }

  /// Get the proxy of the function, which will be then used to dereference the plugin function
  /// ```
  /// use aanyx::runtime::FunctionsRegistry;
  /// 
  /// trait Call {
  ///   fn call( &self );
  /// }
  /// 
  /// fn call_function_or_panic( functions_registry: &FunctionsRegistry<dyn Call>, name: &str ) {
  ///   functions_registry.get( name ).unwrap().call();
  /// }
  /// 
  /// ```
  pub fn get<K: AsRef<str>>( &self, function: K ) -> Option<&FunctionProxy<Prototype>>{ 
    self.functions.get( function.as_ref() )
  }

  /// Get an iterator over all the keys
  /// ```
  /// use aanyx::runtime::FunctionsRegistry;
  /// 
  /// fn print_functions_name<Prototype>( functions_registry: &FunctionsRegistry<Prototype> ) {
  ///   for name in functions_registry.functions(){
  ///     println!( "Function: {}", name );
  ///   }
  /// }
  /// ```
  pub fn functions( &self ) -> Keys< String, FunctionProxy<Prototype> > {
    self.functions.keys()
  }
}


#[cfg(test)]
mod tests {
  use super::FunctionsRegistry;
  
  #[test]
  fn test_plugin(){
    let mut registry = FunctionsRegistry::<dyn std::fmt::Debug>::new();
    let load = unsafe { registry.load( "target/debug/libplugin_test.so" ) };
    assert!( load.is_ok(), "Failed loading the plugin: {}. \nMaybe you forgot to run `cargo build` inside the `plugin-test` directory", load.err().unwrap() );
  }

  #[test]
  fn get_plugin(){
    let mut registry = FunctionsRegistry::<dyn std::fmt::Debug>::new();
    let load = unsafe { registry.load( "target/debug/libplugin_test.so" ) };
    assert!( load.is_ok(), "Failed loading the plugin: {}. \nMaybe you forgot to run `cargo build` inside the `plugin-test` directory", load.err().unwrap() );
    assert!( registry.get( "Test" ).is_some() )
  }
}