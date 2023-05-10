// Re-export of the paste crate to perform macro expansion in plugins
#[doc(hidden)]
pub use paste as plugin_paste;

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct PluginDeclaration<PluginType: ?Sized> {
  pub rustc_version: &'static str,
  pub nyx_version: &'static str,
  pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar<PluginType>),
}

pub trait PluginRegistrar<PluginType: ?Sized> {
  fn register_plugin(&mut self, name: &str, plugin: Box<PluginType>);
}

/// This macro automatically creates all the components that are required by the app to load the plugin and check the compatibility
/// ```
/// use aanyx::{export_plugin, plugin::PluginRegistrar};
/// # mod public_plugin_struct {
/// #   pub struct MyPlugin {}
/// # }
/// 
/// use public_plugin_struct::MyPlugin as MyPlugin;
/// 
/// #[allow(improper_ctypes_definitions)]
/// extern "C" fn register(registrar: &mut dyn PluginRegistrar< MyPlugin >) {
///   registrar.register_plugin("MyPlugin", Box::new( MyPlugin {} ));
/// }
/// export_plugin!( register, MyPlugin );
/// ```
/// 
/// Or even better using traits:
/// ```
/// use aanyx::{ export_plugin, plugin::PluginRegistrar};
/// # mod public_plugin_trait {
/// #   pub trait MyPlugin {
/// #     fn plugin( &self ){}
/// #   }
/// # }
/// 
/// use public_plugin_trait::MyPlugin as MyPluginTrait;
/// struct MyPlugin;
/// impl MyPluginTrait for MyPlugin {
///  // Trait implentation goes here
/// };
/// 
/// #[allow(improper_ctypes_definitions)]
/// extern "C" fn register(registrar: &mut dyn PluginRegistrar<dyn MyPluginTrait>) {
///   registrar.register_plugin("MyPlugin", Box::new(MyPlugin));
/// }
/// export_plugin!( register, dyn MyPluginTrait );
/// ```
/// 
/// ## Limitations
/// See [`import_plugin's limitations`](crate::import_plugin#limtations).
#[macro_export]
macro_rules! export_plugin {
  ($register:expr, dyn $plugin_type:ident ) => {
    $crate::plugin::plugin_paste::paste! {
    #[doc(hidden)]
    #[no_mangle]
    pub static [<plugin_declaration _ dyn _  $plugin_type >]: $crate::plugin::PluginDeclaration::<dyn $plugin_type> = $crate::plugin::PluginDeclaration {
      rustc_version: $crate::RUSTC_VERSION,
      nyx_version: $crate::CORE_VERSION,
      register: $register,
    };
    }
  };
  ($register:expr, $plugin_type: ident) => {
    $crate::plugin::plugin_paste::paste! {
    #[doc(hidden)]
    #[no_mangle]
    pub static [<plugin_declaration _  $plugin_type >]: $crate::plugin::PluginDeclaration::<$plugin_type> = $crate::plugin::PluginDeclaration {
      rustc_version: $crate::RUSTC_VERSION,
      nyx_version: $crate::CORE_VERSION,
      register: $register,
    };
    }
  };
}

/// This is a macro that generates the name of the plugin declaration in a module.
/// Using this makes easier to check type compatibility between plugins and host systems, 
/// because if the plugin doesn't support the requested plugin type then no declaration will be found.
/// ```
/// use aanyx::import_plugin;
/// assert_eq!( import_plugin!( MyPlugin ), b"plugin_declaration_MyPlugin");
/// assert_eq!( import_plugin!( dyn MyPluginTrait ), b"plugin_declaration_dyn_MyPluginTrait");
/// ```
/// 
/// ## Limitations
/// Because of Rust macros limitations a type cannot contains `::`. 
/// This means that this is invalid: 
/// ```compile_fail, no_run
/// use aanyx::import_plugin;
/// import_plugin!( myplugin::MyPlugin )
/// ```
/// A workaround is the following:
/// ```
/// # mod myplugin { pub struct MyPlugin {} }
/// use myplugin::MyPlugin as MyPlugin;
/// use aanyx::import_plugin;
/// 
/// import_plugin!( MyPlugin );
/// ```
/// Also using a type alias makes a result not compatible between host and plugin.
/// ```
/// # mod myplugin { pub struct MyPlugin {} }
/// use aanyx::import_plugin;
/// use myplugin::MyPlugin as MyPlugin;
/// type MyAlias = MyPlugin;
/// 
/// assert_ne!( import_plugin!( MyAlias ), import_plugin!( MyPlugin ) );
/// ```
#[macro_export]
macro_rules! import_plugin {
  ( dyn $plugin_type:ident ) => {
     ( concat!("plugin_declaration_dyn_", stringify!( $plugin_type )) ).as_bytes()
  };
  ( $plugin_type:ident ) => {
    ( concat!("plugin_declaration_", stringify!( $plugin_type )) ).as_bytes()
  };
}