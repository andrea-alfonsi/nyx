use std::fmt::Debug;

use super::{FromRegistry, Handler};

/// The `Service` is the starting point of the static dispatch fuctionality of the crate.
/// It allows any function and closure to became a statically dispatched function. 
/// It supports up to 15 arguments.
/// 
/// # How it works
/// Each service contains a `Handler` with the right input arguments and outputs.
/// Take for example `Handler<..., (String, u8), bool>`. This means that the handler represents a function like `fn can_drive(name: String, age: u8) -> bool`.
/// The handler then implements a custom `call` methods that extracts the tuple `(String, u8)` from the given `Registry` using the function ( String::from( &registry ), u8::from( &registry ) ),
/// and then calls the target function  with the custom parameters
/// 
/// ```
/// use nyx::compile_time::Service;
/// let return_always_true = ||{true};
/// let registry = ();
/// let my_service = Service::new( Box::new( return_always_true ) );
/// assert!( my_service.call( &registry ) )
/// ```
pub struct Service<Registry, Return> {
  handler: Box<dyn Fn(&Registry) -> Return>
}

impl<Registry: 'static, Return: 'static> Service<Registry, Return> {

  /// Create a new `Service` from a `Box` containing a function or a closure.
  /// ```
  /// use nyx::compile_time::Service;
  /// let return_always_true = ||{true};
  /// let registry = ();
  /// let my_service = Service::new( Box::new( return_always_true ) );
  /// # assert!( my_service.call( &registry ) )
  /// ```
  pub fn new<Args: FromRegistry<Registry> + 'static>( handler: Box<dyn Handler<Args, Return = Return>> ) -> Self {
    Self { handler: Box::new( move |registry: &Registry| {
      handler.call_with( Args::from( registry ))
    })}
  }

  /// Replace the handler inside a service with another
  /// ```
  /// use nyx::compile_time::Service;
  /// let return_always_true = ||{true};
  /// let return_always_false = ||{false};
  /// let registry = ();
  /// let mut my_service = Service::new( Box::new( return_always_true ) );
  /// my_service.set_handler( Box::new( return_always_false ) );
  /// # assert!( !my_service.call( &registry ) )
  /// ```
  pub fn set_handler<Args: FromRegistry<Registry> + 'static>( &mut self, handler: Box<dyn Handler<Args, Return = Return>>){
    self.handler = Box::new( move |registry: &Registry| { 
      handler.call_with( Args::from( registry) ) 
    } );
  }

  /// Run the function inside the service, given the `Registry` containing all the data
  /// ```
  /// use nyx::compile_time::Service;
  /// let return_always_true = ||{true};
  /// let registry = ();
  /// let my_service = Service::new( Box::new( return_always_true ) );
  /// assert!( my_service.call( &registry ) )
  /// ```
  pub fn call( &self, registry: &Registry ) -> Return {
    (self.handler)( registry )
  }
}

impl<Registry, Return> Debug for Service<Registry, Return>{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Service< ({}) -> {} >" , std::any::type_name::<Registry>(), std::any::type_name::<Return>())
  }
}

#[cfg(test)]
mod tests {
  use super::Service; 

  fn noop() -> bool { true }
  fn noop1( _: () ) -> bool { true }
  fn noop2( _: (), _: () ) -> bool { true }
  fn noop12( _: (), _: (), _: (), _: (), _: (), _: (),
             _: (), _: (), _: (), _: (), _: (), _: ()) -> bool { true }

  #[test]
  fn test_fn_as_service(){
    let registry = ();
    let service0 = Service::new( Box::new( noop ) );
    let service1 = Service::new( Box::new( noop1 ));
    let service2 = Service::new( Box::new( noop2 ));
    let service12 = Service::new( Box::new( noop12 ));

    assert!( service0.call( &registry ));
    assert!( service1.call( &registry ));
    assert!( service2.call( &registry ));
    assert!( service12.call( &registry ));
  }

  #[test]
  fn test_closure_as_service(){
    let always_true0 = ||{ true };
    let always_true1 = | _: () |{ true };

    let registry = ();
    let service0 = Service::new( Box::new( always_true0 ));
    let service1 = Service::new( Box::new( always_true1 ));

    assert!( service0.call( &registry ));
    assert!( service1.call( &registry ));
  }
}