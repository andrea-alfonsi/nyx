/// The Handler is the trait responsable of exploding the `Args` into the correct number of arguments for the function.
/// The magic of extraction from `Registry` into the right `Args` is done inside the `Service` struct.
/// The trait is automatically implemented for all the functions up to 15 arguements
/// ```
/// use aanyx::compile_time::handler::Handler;
/// type Args = (String, u8);
/// let args: Args = (String::from("Andrea"), 120u8);
/// fn is_old_enough( name: String, age: u8) -> bool {
///   age > 100
/// }
/// let func: Box<dyn Handler<Args, Return = bool> > = Box::new( is_old_enough );
/// assert_eq!(func.call_with( args ), true);
/// ```
pub trait Handler<Args>{
  type Return;
  fn call_with( &self, args: Args) -> Self::Return;
}

macro_rules! impl_handler_for {
  ( $( $x:ident ),* ) => {
    impl<Func, FnReturnType, $( $x ),*> Handler<($($x),*)> for Func where Func: Fn($($x),*) -> FnReturnType {
      type Return = FnReturnType;
      #[allow(non_snake_case)]
      fn call_with( &self, ($($x),*): ($($x),*)) -> Self::Return {
          (self)($($x),*)
      }
    }
  };
}

impl<F, A, FnReturnType> Handler<(A, )> for F where F: Fn(A) -> FnReturnType {
  type Return = FnReturnType;
  fn call_with( &self, (a, ): (A, )) -> Self::Return {
      (self)(a)
  }
}

impl<F, FnReturnType> Handler<()> for F where F: Fn() -> FnReturnType {
  type Return = FnReturnType;
  fn call_with( &self, _: ()) -> Self::Return {
      (self)()
  }
}


impl_handler_for!( A, B );
impl_handler_for!( A, B, C );
impl_handler_for!( A, B, C, D );
impl_handler_for!( A, B, C, D, E );
impl_handler_for!( A, B, C, D, E, F );
impl_handler_for!( A, B, C, D, E, F, G, H );
impl_handler_for!( A, B, C, D, E, F, G, H, I );
impl_handler_for!( A, B, C, D, E, F, G, H, I, J );
impl_handler_for!( A, B, C, D, E, F, G, H, I, J, K );
impl_handler_for!( A, B, C, D, E, F, G, H, I, J, K, L );
impl_handler_for!( A, B, C, D, E, F, G, H, I, J, K, L, M );
impl_handler_for!( A, B, C, D, E, F, G, H, I, J, K, L, M, N );
impl_handler_for!( A, B, C, D, E, F, G, H, I, J, K, L, M, N, O );