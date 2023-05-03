# Nyx 
This Rust Cargo package is designed to facilitate modularity in Rust applications. 
With this crate, you can easily create a modular Rust application that can dynamically load functions and plugins at runtime. 
This makes it easy to extend and customize your application without having to recompile or modify the core code.
The package is split into two parts: one that can convert functions into services at runtime and the other that simplifies the process of importing plugins and functions at runtime.

## Compile-time

The first part of this crate allows for the creation of services at runtime. 
A service is a function that can automatically extract arguments from a registry, making it possible to switch services at runtime.
Overall, using services from this crate provides a flexible and efficient way to create modular applications that can be easily customized and extended at runtime.
A performance analysis run with `criterion` can be found [here](./src/compile_time/README.md/#performance-analysis)

### HOW-TOs
* [Implement custom registry](./src/compile_time/README.md#how-to-use-custom-registry)

### TODOS

* Add support for async functions

## Runtime
The second part of this crate allows to create a consistend and safe ecosystem of plugins.
Plugins are essentially add-ons or extensions that can be loaded into the application at runtime. 
They can add new features, functionality, or services to the application without requiring a recompilation or modification of the core code. 
This makes it easy to create customized versions of the application for specific use cases, or to add new features without having to modify the core codebase.

Additionally, plugins can enable the development of a rich ecosystem of third-party extensions, which can enhance the capabilities of the application and drive innovation. This can lead to a more vibrant and active community around the application, which can help to promote its adoption and usage.

### HOW-TOs
* [Implement a plugin interface](./src/runtime/README.md#how-to-define-an-interface)
