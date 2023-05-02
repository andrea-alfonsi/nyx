# How to define an interface
The interface must follow the requirements called "object safe", that can be found [here](https://doc.rust-lang.org/reference/items/traits.html#object-safety)

```rust
trait MyPlugin {
  fn call( &self );
  fn other_call( self: Rc<Self>, data: usize ) -> bool;
}

let functions_registry = nyx::runtime::FunctionsRegistry::<dyn MyPlugin>::new();
```