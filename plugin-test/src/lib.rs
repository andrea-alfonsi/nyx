
use nyx::{export_plugin, runtime::plugin::*};

export_plugin!(register);

#[derive(Debug)]
struct Test;

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function("Test", Box::new(Test));
}