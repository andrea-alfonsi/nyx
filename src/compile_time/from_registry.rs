pub mod hashmap;
pub use hashmap::*;

/// This trait is used to extract a type from a `Registry`. 
/// If a `Registry` not implemented by default is used, then you should implement this trait for all the used traits.
/// ```
/// use aanyx::compile_time::FromRegistry;
/// # #[derive(Debug, PartialEq)]
/// # struct MyStruct;
/// # impl FromRegistry<()> for MyStruct {
/// #   fn from( _: &() ) -> Self {
/// #     MyStruct
/// #   }
/// # }
/// let registry = ();
/// let my_struct: MyStruct = <MyStruct as FromRegistry<()>>::from( &registry );
/// # assert_eq!( my_struct, MyStruct );
/// ```
pub trait FromRegistry<Registry> {
  fn from( registry: &Registry ) -> Self;
}

macro_rules! impl_from_registry {
  ( $( $x:ident ),* ) => {
    impl<Registry, $( $x: FromRegistry<Registry>),* > FromRegistry<Registry> for ( $( $x ),* ) {
      fn from( registry: &Registry ) -> Self {
          ( $( $x::from(registry) ),* )
      }
    }
  };
}

impl<Registry> FromRegistry<Registry> for () {
  fn from( _: &Registry ) -> Self {
    ()
  }
}

impl<A: FromRegistry<Registry>, Registry> FromRegistry<Registry> for (A, ) {
    fn from( registry: &Registry ) -> Self {
        ( A::from( registry ), )
    }
}

impl_from_registry!( A, B );
impl_from_registry!( A, B, C );
impl_from_registry!( A, B, C, D );
impl_from_registry!( A, B, C, D, E );
impl_from_registry!( A, B, C, D, E, F );
impl_from_registry!( A, B, C, D, E, F, G, H );
impl_from_registry!( A, B, C, D, E, F, G, H, I );
impl_from_registry!( A, B, C, D, E, F, G, H, I, J );
impl_from_registry!( A, B, C, D, E, F, G, H, I, J, K );
impl_from_registry!( A, B, C, D, E, F, G, H, I, J, K, L );
impl_from_registry!( A, B, C, D, E, F, G, H, I, J, K, L, M );
impl_from_registry!( A, B, C, D, E, F, G, H, I, J, K, L, M, N );
impl_from_registry!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O );


#[cfg(test)]
mod tests {
  use super::FromRegistry;

  #[test]
  fn test_custom_from_registry(){
    #[derive(PartialEq, Debug)]
    struct S;
    impl FromRegistry<()> for S {
      fn from( _: &() ) -> Self {
        S
      }
    }

    assert_eq!(S, <S as FromRegistry<()>>::from( &() ))
  }
}