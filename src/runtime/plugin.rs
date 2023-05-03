/// This structs contains all the informations that can be automatically extracted at compile-time
/// and is used by the app to check the compatibility of the plugin like the rust version, this package versions, ecc...
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct PluginDeclaration {
  pub rustc_version: &'static str,
  pub nyx_version: &'static str,
  pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

/// This trait is the juction between the plugins and the app.
/// 
/// # The `Prototype` generic
/// In the app the `Prototype` should be specified to ensure all the plugins are compatible, 
/// this is can also be done in the plugin if the `Prototype` is made available.
/// 
pub trait PluginRegistrar<Prototype: ?Sized = dyn std::any::Any> {
  fn register_function(&mut self, name: &str, function: Box<Prototype>);
}

/// This macro automatically creates all the components that are required by the app to load the plugin and check the compatibility
/// ```
/// use aanyx::{runtime::plugin::PluginRegistrar, export_plugin};
/// 
/// struct MyPlugin;
/// 
/// extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
///   registrar.register_function("MyPlugin", Box::new(MyPlugin));
/// }
/// export_plugin!( register );
/// ```
/// 
/// Or even better using traits:
/// ```
/// use aanyx::{runtime::plugin::PluginRegistrar, export_plugin};
/// # mod public_plugin_trait {
/// #   pub trait MyPlugin {
/// #     fn plugin( &self ){}
/// #   }
/// # }
/// 
/// struct MyPlugin;
/// impl public_plugin_trait::MyPlugin for MyPlugin {
///  // Trait implentation goes here
/// };
/// 
/// extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
///   registrar.register_function("MyPlugin", Box::new(MyPlugin));
/// }
/// export_plugin!( register );
/// ```
#[macro_export]
macro_rules! export_plugin {
  ($register:expr) => {
    #[doc(hidden)]
    #[no_mangle]
    pub static plugin_declaration: $crate::runtime::PluginDeclaration = $crate::runtime::PluginDeclaration {
      rustc_version: $crate::RUSTC_VERSION,
      nyx_version: $crate::CORE_VERSION,
      register: $register,
    };
  };
}