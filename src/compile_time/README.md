# How to use custom registry
This library implements automatically some basic registry, but for more advanced projects or requirements custom registry can be more performant.
To use a custom registry you need to implement the `FromRegistry` trait for all the types that you are going to use, (except for `tuples` that are automatically generated).

### Example 
We want to implement the trait for a the registry `MyRegistry`.

```rust
struct MyRegistry {
  age: u8
};

impl FromRegistry<MyRegistry> for String {
  fn from( registry: &Registry ) -> Self {
    String::from( "THE QUICK BROWN FOX..." )
  }
}

impl FromRegistry<MyRegistry> for u8 {
  fn from( registry: &Registry ) -> Self {
    registry.age
  }
}
```

although probably is sufficient implement the trait for `Rc` and `Arc`, to cover almost all use.

```rust
impl<F> FromRegistry<MyRegistry> for Rc<F> { ... }
impl<F> FromRegistry<MyRegistry> for Arc<F> { ... }
```