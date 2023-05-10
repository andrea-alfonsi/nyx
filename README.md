# Nyx 
This Rust Cargo package is designed to facilitate modularity in Rust applications. 
With this crate, you can easily create a modular Rust application that can dynamically load functions and plugins at runtime. 
This makes it easy to extend and customize your application without having to recompile or modify the core code.

This crate contains 4 module:
* host
* PDK
* plugin
* system
  
## Host
The `host` contains the tools and utilities that a develoepr should use in the main app, when creating a plugin manager.

## PDK
The `pdk` contains some utilities and tools to test plugins locally before releasing them. It implements some structures that inspect what a plugin is doing

## Plugin
The `plugin` module contains the macros and the structs definitions to allow the plugin manager to understand the structure of the plugin

## System
This crate uses the word `system` meaning a function that accepts any number of arguments. This simplifies the structure of the code becuse understanding which arguemnts should be bessedt oa function becomes a task of he compiler


# Safety
This crate doesn't use unsafe functions except in `pdk` for testing purpose


# Examples
Examples of how to use the crate can be found in [Examples](/examples/)
