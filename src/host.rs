//! This is the module containing all the definitions of the structs and trait used by the host system.
//! We refer as `Host system` meaning the part of the application that is responsible to load, manage and call plugins.
//! The traits makes no distinction between dynamic and static dispatch, so restricting to a carticular case is a job of the developer.


use std::alloc::System;
// Currently the default global allocator is unspecified. Libraries, however, 
// like cdylibs and staticlibs are guaranteed to use the System by default.
#[global_allocator]
static ALLOCATOR: System = System;

pub trait PluginLoader<PluginId, PluginType> {
  fn into_plugin( self ) -> (PluginId, PluginType);
}

/// Allow the plugin manager the possibility to load packages.
/// So a struct that doesn't implement this trait won't ba able to load plugins.
/// You may don't want to implement this trait to have more control over loaded plugins, 
/// so once the plugins are loaded no more plugins can be loaded from other functions.
/// 
/// ```
/// use aanyx::host::PluginManagerLoad;
/// # use aanyx::host::PluginManagerGet;
/// use aanyx::host::PluginLoader;
/// use std::collections::HashMap;
/// 
/// struct MyPluginLoader { name: String }
/// impl PluginLoader<String, String> for MyPluginLoader {
///   fn into_plugin( self ) -> (String, String) { 
///     // Simulates loading somethig and perform some calculations
///     let mut plugin = self.name.clone();
///     plugin.push_str("__");
/// 
///     ( self.name.clone(), plugin) 
///   }
/// }
/// 
/// struct MyPluginManager { plugins: HashMap<String, String>}
/// impl PluginManagerLoad<String, String, ()> for MyPluginManager {
///   fn load( &mut self, new_plugin: impl PluginLoader<String, String>) -> Result<(), ()>{ 
///     let (id, plugin) = new_plugin.into_plugin();
///     self.plugins.insert( id, plugin );
///     Ok(())
///   }
/// }
/// # impl PluginManagerGet<String, String>for MyPluginManager { 
/// #   fn get( &self, plugin: &String ) -> Option<&String>{
/// #     self.plugins.get( plugin )
/// #   }
/// # }
/// 
/// let plugin1 = MyPluginLoader { name: String::from("Plugin_1")};
/// let mut plugin_manager = MyPluginManager { plugins: HashMap::new() };
/// 
/// plugin_manager.load( plugin1 );
/// plugin_manager.load( MyPluginLoader { name: String::from("Plugin_2")} );
/// 
/// # assert_eq!( plugin_manager.get( &String::from("Plugin_1") ), Some(&String::from("Plugin_1__")) );
/// # assert_eq!( plugin_manager.get( &String::from("Plugin_2") ), Some(&String::from("Plugin_2__")) );
/// 
/// ```
/// ## Safety
/// All functions are marked safe although some implementation may use `libloading` or other unsafe crates.
/// The [`PluginManagerLoad`] should then take care of the unsafeties itself depending on what crates and methods it uses.
/// This allows to make the trait also available for static plugins that doesn't require runtime loading.
pub trait PluginManagerLoad<PluginId, PluginType, ManagerLoadError> 
{
  fn load( &mut self, new_plugin: impl PluginLoader<PluginId, PluginType>) -> Result<(), ManagerLoadError>;
}

/// Get the plugins loaded in the PluginManager.
/// This trait is independent from the loader, in fact you may want to implement a plugin manager that serves 
/// only as container for a group of plugins and extract the  required ones.
/// ```
/// use aanyx::host::PluginManagerGet;
/// use std::collections::HashMap;
/// 
/// struct MyPluginManager { plugins: HashMap<String, String>}
/// impl PluginManagerGet<String, String> for MyPluginManager {
///   fn get( &self, plugin: &String ) -> Option<&String> {
///     self.plugins.get( plugin )
///   }
/// }
/// 
/// impl MyPluginManager {
///   pub fn new() -> Self {
///     let mut plugins = HashMap::new();
///     plugins.insert( String::from("PluginName"), String::from("PlugintType") );
///     Self { plugins }
///   }
/// }
/// 
/// let plugin_manager = MyPluginManager::new();
/// assert_eq!( plugin_manager.get( &String::from("PluginName") ), Some(&String::from("PlugintType")))
/// ```
pub trait PluginManagerGet<PluginId, PluginType> {
  fn get( &self, plugin: &PluginId) -> Option<&PluginType>;
}

/// Unload a plugin
/// ```
/// use aanyx::host::{PluginManagerUnload};
/// use std::collections::HashMap;
/// struct MyPluginManager { pub plugins: HashMap<String, String>}
/// 
/// impl PluginManagerUnload<String, String, ()> for MyPluginManager {
///   fn unload( &mut self, old_plugin: &String ) -> Result<(), ()>{
///     assert!(  self.plugins.remove( old_plugin ).is_some() );
///     Ok(())
///   }
/// }
/// 
/// let mut my_plugin_manager = MyPluginManager { plugins: HashMap::new() };
/// my_plugin_manager.plugins.insert( String::from("MyPlugin"), String::from("Plugin__"));
/// assert!( my_plugin_manager.unload ( &String::from("MyPlugin") ).is_ok());
/// 
/// ```
pub trait PluginManagerUnload<PluginId, PluginType, ManagerUnloadError>
{
  fn unload( &mut self, old_plugin: &PluginId ) -> Result<(), ManagerUnloadError>;
}


/// Allow the manager to overwrite a loaded plugin with another version of it
/// ```
/// use aanyx::host::{PluginManagerUnload, PluginManagerLoad, PluginManagerReload, PluginLoader};
/// use std::collections::HashMap;
/// 
/// struct MyPluginLoader { name: String }
/// impl PluginLoader<String, String> for MyPluginLoader {
///   fn into_plugin( self ) -> (String, String) { 
///     // Simulates loading somethig and perform some calculations
///     let mut plugin = self.name.clone();
///     plugin.push_str("__");
/// 
///     ( self.name.clone(), plugin) 
///   }
/// }
/// 
/// struct MyPluginManager { pub plugins: HashMap<String, String>}
/// impl PluginManagerLoad<String, String, ()> for MyPluginManager {
///   fn load( &mut self, new_plugin: impl PluginLoader<String, String>) -> Result<(), ()>{ 
///     let (id, plugin) = new_plugin.into_plugin();
///     self.plugins.insert( id, plugin );
///     Ok(())
///   }
/// }
/// impl PluginManagerUnload<String, String, ()> for MyPluginManager { 
///   fn unload( &mut self, plugin: &String ) -> Result<(), ()>{
///     self.plugins.remove( plugin );
///     Ok(())
///   }
/// }
/// 
/// impl PluginManagerReload<String, String, ()> for MyPluginManager {
///   fn reload( &mut self, plugin: &String ) -> Result<(), ()>{
///     let mut new_plugin = MyPluginLoader { name: plugin.to_string() }.into_plugin();
///     new_plugin.1.push_str("reloaded");
///     *self.plugins.get_mut( plugin ).unwrap() = new_plugin.1;
///     Ok(())
///   }
/// }
/// 
/// let mut my_plugin_manager = MyPluginManager { plugins: HashMap::new() };
/// let my_plugin1 = MyPluginLoader { name: String::from("MyPlugin")};
/// my_plugin_manager.load( my_plugin1 );
/// 
/// assert_eq!( my_plugin_manager.plugins.get( "MyPlugin" ).unwrap(), &String::from("MyPlugin__"));
/// 
/// my_plugin_manager.reload( &String::from("MyPlugin") );
/// 
/// assert_eq!( my_plugin_manager.plugins.get( "MyPlugin" ).unwrap(), &String::from("MyPlugin__reloaded"));
/// ```
pub trait PluginManagerReload<PluginId, PluginType, ManagerReloadError> {
  /// Replace a plugin with a another version
  /// If the manager cannot replace the plugin by its name replace the [`PluginManagerReload::reload_as_new`] and set this one as unimplemented:
  /// ```
  /// use aanyx::host::PluginManagerReload;
  /// # use std::collections::HashMap;
  /// # use aanyx::host::PluginLoader;
  /// # struct MyPluginManager { plugins: HashMap<(), ()> }
  /// # struct MyPluginLoader{}
  /// # impl PluginLoader<(),()> for MyPluginLoader {fn into_plugin( self ) -> ((),()) { ((), () )}}
  /// 
  /// impl PluginManagerReload<(), (), ()> for MyPluginManager {
  ///   fn reload( &mut self, plugin_name: &() ) -> Result<(), ()>{
  ///     unimplemented!();
  ///   }
  ///   fn reload_as_new( &mut self, _new_plugin: impl PluginLoader<(), ()>, old_plugin: &()) -> Result<(), ()> {
  ///     // Implement reload here
  ///     Ok(())
  ///   }
  /// }
  /// # let mut my_plugin_manager = MyPluginManager { plugins: HashMap::new() };
  /// let should_panic = std::panic::catch_unwind( move || my_plugin_manager.reload( &() ));
  /// assert!( should_panic.is_err(), "my_plugin_manager.reload should have been panicked" );
  /// ```
  fn reload( &mut self, plugin: &PluginId ) -> Result<(), ManagerReloadError>;

  /// This function falls back into the [`PluginManagerReload::reload`] function if not overwritten.
  /// Replace default definition if the manager has no way to understand which plugin requires a reload from its id
  /// ## Doesn't require overwrite example
  /// ``` 
  /// use aanyx::host::PluginManagerReload;
  /// # use std::collections::HashMap;
  /// # fn load_library_from_file_by_name( _: &String ) -> String { String::from("Test") }
  /// # struct MyPluginManager { plugins: HashMap<String, String> }
  /// 
  /// // PluginId is equal to the shared object file name of the plugin
  /// type PluginId = String;
  /// 
  /// impl PluginManagerReload<String, (), ()> for MyPluginManager {
  ///   fn reload( &mut self, plugin_name: &String ) -> Result<(), ()>{
  ///     self.plugins.remove( plugin_name );
  ///     let library = load_library_from_file_by_name( plugin_name );
  ///     self.plugins.insert( plugin_name.to_string(), library );
  ///     Ok(())
  ///   }
  /// }
  /// ```
  fn reload_as_new( &mut self, _new_plugin: impl PluginLoader<PluginId, PluginType>, old_plugin: &PluginId) -> Result<(), ManagerReloadError> {
    self.reload(old_plugin)
  }
}
