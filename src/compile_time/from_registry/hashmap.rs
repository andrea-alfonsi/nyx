//! Implement the `FromRegistry` trait for the type `HashMap< TypeId, Box<dyn std::any::Any>>`
//! ```
//! use std::{collections::HashMap, any::TypeId, rc::Rc};
//! use aanyx::compile_time::{Service, from_registry::HashMapType};
//! let mut map: HashMapType = HashMap::new();
//! let data1 = Rc::new( String::from("Andrea") );
//! let data2 = Rc::new( 120u8 );
//! map.insert( TypeId::of::<Rc<String>>(), Box::new(data1) );
//! map.insert( TypeId::of::<Rc<u8>>(), Box::new(data2) );
//! 
//! let f = |name: Rc<String>, age: Rc<u8>| -> bool { *age > 119 };
//! let service = Service::new( Box::new( f ) );
//! assert!(service.call( &map ));
//! ```
use std::{collections::HashMap, any::TypeId, rc::Rc, sync::Arc};
use super::FromRegistry;

pub type HashMapType = HashMap< TypeId, Box<dyn std::any::Any>>;

impl<F> FromRegistry<HashMapType> for Rc<F> 
where F: 'static{
  fn from( registry: &HashMapType ) -> Self {
    let rc = registry.get( &std::any::TypeId::of::<Rc<F>>() ).unwrap().downcast_ref::<Rc<F>>().unwrap();
    Rc::clone( &rc )
  }
}

impl<F> FromRegistry<HashMapType> for Arc<F> 
where F: 'static{
  fn from( registry: &HashMapType ) -> Self {
    let arc = registry.get( &std::any::TypeId::of::<Arc<F>>() ).unwrap().downcast_ref::<Arc<F>>().unwrap();
    Arc::clone( &arc )
  }
}