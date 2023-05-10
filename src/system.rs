//! A `Registry` is to be intended as a big container from which we can extract data based on their type. 
//! A simple example is the `HashMap<TypeId, Box<Any>>`
//! A [`System`](crate::system::System) is any function that accepts any number (up to 15 because of implementation details) 
//! of arguments supporting the `FromRegistry` trait.

/// Defines how a custom datatype can be extracted from a registry to be passed into a system. 
/// ```
/// use aanyx::system::FromRegistry;
/// # struct MyRegistry {}
/// # struct MyDataType {}
/// 
/// impl MyDataType { pub fn return_true( &self ) -> bool { true } }
/// 
/// impl FromRegistry< MyRegistry > for MyDataType {
///   fn from_registry( registry: &MyRegistry ) -> Self  {
///     // Logic to extract MyDatatype
///     MyDataType {}
///   }
/// }
/// # let my_registry = MyRegistry {};
/// assert!( MyDataType::from_registry( &my_registry ).return_true() )
/// ```
/// It is automatically implemented for tuples up to 15 parameters. This can anyway be extend by nesting tuples.
/// ```
/// use aanyx::system::FromRegistry;
/// # struct MyRegistry {}
/// # let my_registry = MyRegistry {};
/// # trait MyDefault:Default {}
/// # impl FromRegistry< MyRegistry > for String {  fn from_registry( _registry: &MyRegistry ) -> Self { Self::default() } } 
/// # impl FromRegistry< MyRegistry > for u8 {  fn from_registry( _registry: &MyRegistry ) -> Self { Self::default() } } 
/// # impl FromRegistry< MyRegistry > for usize {  fn from_registry( _registry: &MyRegistry ) -> Self { Self::default() } } 
/// 
/// // These are equivalents
/// let (a0, b0, c0 /*, others... */) = <(String, u8, usize /*, Others... */)>::from_registry( &my_registry );
/// let ((a1, b1), c1, /* others... */ ) = <((String, u8), usize /*, Others... */)>::from_registry( &my_registry );
/// 
/// // In general that's not true. It depends on the registry. This example makes them all equals for simplicity
/// assert_eq!( a0, a1 );
/// assert_eq!( b0, b1 );
/// assert_eq!( c0, c1 );
/// ```
pub trait FromRegistry<Registry> {
  // TODO: make the function argument be mutable?
  fn from_registry( registry: &Registry ) -> Self;
}

macro_rules! impl_from_registry {
  ( $( $x:ident ),* ) => {
    impl<Registry, $( $x: FromRegistry<Registry>),* > FromRegistry<Registry> for ( $( $x ),* ) {
      fn from_registry( registry: &Registry ) -> Self {
          ( $( $x::from_registry(registry) ),* )
      }
    }
  };
}

impl<Registry> FromRegistry<Registry> for () {
  fn from_registry( _: &Registry ) -> Self {
    ()
  }
}

impl<A: FromRegistry<Registry>, Registry> FromRegistry<Registry> for (A, ) {
    fn from_registry( registry: &Registry ) -> Self {
        ( A::from_registry( registry ), )
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


/// A system is a trait that allows a function to be called giving only a registry and letting the compiler
/// figure out what needs to be extracted from that registry in order to call the function
/// ```
/// use aanyx::system::System;
/// use std::rc::Rc;
/// 
/// struct Data<T>{ data: Rc<T> }
/// 
/// # use aanyx::system::FromRegistry;
/// # impl FromRegistry<(Rc<Person>, Location)> for Data<Person> { fn from_registry( (person, _) :&(Rc<Person>, Location)) -> Self { Data { data: Rc::clone(person) } }}
/// // Simulate other data
/// struct Location {}
/// struct Person{ name: String, age: u8 }
/// fn old_enough( person: Data<Person> ) -> bool { person.data.age > 30 }
/// 
/// let registry0 = ( Rc::new(Person{ name: String::from("Alice"), age: 24u8 }),  Location { /* Location data */ }) ;
/// let registry1 = ( Rc::new(Person{ name: String::from("Bob"), age: 31u8 }),    Location { /* Location data */ });
/// let registry2 = ( Rc::new(Person{ name: String::from("Charlie"), age: 1u8 }), Location { /* Location data */ });
/// 
/// assert_eq!( old_enough.apply( &registry0 ), false );
/// assert_eq!( old_enough.apply( &registry1 ), true );
/// assert_eq!( old_enough.apply( &registry2 ), false );
/// ```
/// ## Closures
/// A closure can be made into a `System` only if the closure accepts no arguments
/// ```
/// use aanyx::system::System;
/// 
/// fn always_true() -> bool { true }
/// let always_true_wrapper = ||{ always_true.apply( &()) };
/// 
/// // The &() is required because the `System.apply` expects an argument although the closure has 0 arguments
/// assert!( always_true_wrapper.apply( &() ) );
/// ```
/// 
/// ```
/// use aanyx::system::System;
/// use std::rc::Rc;
/// let registry = Rc::new(());
/// let registry_copy = Rc::clone( &registry );
/// 
/// fn always_true() -> bool { true }
/// let always_true_wrapper = move ||{ always_true.apply( &registry_copy ) };
/// 
/// assert!( always_true_wrapper.apply( &registry ))
/// ```
/// ## Performance analysis
/// Benchamrks performed suggests that the overhead of calling a system insted of the original function is negligible.
/// 
/// ## Async programming
/// Actually the system supports async functions, but arguments extraction from registry is not async.
/// 
/// ## Safety
/// Calling a unsafe function through the `System` trait will not generate any new unsafeties, but the function will still be unsafe.
pub trait System<Registry, Args: FromRegistry<Registry>>{
  type Return;
  fn apply( &self, registry: &Registry ) -> Self::Return;
}

macro_rules! impl_system_for {
  ( $( $x:ident ),* ) => {
    impl<Registry, Func, FnReturnType, $( $x:FromRegistry<Registry> ),*> System<Registry, ($($x),*)> for Func where Func: Fn($($x),*) -> FnReturnType {
      type Return = FnReturnType;
      #[allow(non_snake_case)]
      fn apply( &self, registry: &Registry ) -> Self::Return {
        let ($($x),*) =  ($($x::from_registry( registry )),*);
        (self)($($x),*)
      }
    }
  };
}

impl<Registry, F, A: FromRegistry<Registry>, FnReturnType> System<Registry, (A, )> for F where F: Fn(A) -> FnReturnType {
  type Return = FnReturnType;
  fn apply( &self, registry: &Registry) -> Self::Return {
    let a = A::from_registry( registry );
    (self)(a)
  }
}

impl<Registry, F, FnReturnType> System<Registry, ()> for F where F: Fn() -> FnReturnType {
  type Return = FnReturnType;
  fn apply( &self, _: &Registry) -> Self::Return {
      (self)()
  }
}


impl_system_for!( A, B );
impl_system_for!( A, B, C );
impl_system_for!( A, B, C, D );
impl_system_for!( A, B, C, D, E );
impl_system_for!( A, B, C, D, E, F );
impl_system_for!( A, B, C, D, E, F, G, H );
impl_system_for!( A, B, C, D, E, F, G, H, I );
impl_system_for!( A, B, C, D, E, F, G, H, I, J );
impl_system_for!( A, B, C, D, E, F, G, H, I, J, K );
impl_system_for!( A, B, C, D, E, F, G, H, I, J, K, L );
impl_system_for!( A, B, C, D, E, F, G, H, I, J, K, L, M );
impl_system_for!( A, B, C, D, E, F, G, H, I, J, K, L, M, N );
impl_system_for!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O );


