use std::{env, rc::Rc};
use aanyx::system::{FromRegistry, System};
use std::time::SystemTime;

struct Person {
  name: Rc<String>, 
  age: Rc<u8>,
  location: Rc<(f64, f64)>,
  birthday: Rc<SystemTime>
}

struct Data<T> {
  data: Rc<T>
}

impl FromRegistry<Person> for Data<String> {
  fn from_registry( registry: &Person ) -> Self {
    Self{ data: Rc::clone(&registry.name ) }
  }
}

impl FromRegistry<Person> for Data<u8> {
  fn from_registry( registry: &Person ) -> Self {
    Self{ data: Rc::clone(&registry.age ) }
  }
}

impl FromRegistry<Person> for Data<(f64, f64)> {
  fn from_registry( registry: &Person ) -> Self {
    Self{ data: Rc::clone(&registry.location ) }
  }
}

impl FromRegistry<Person> for Data<SystemTime> {
  fn from_registry( registry: &Person ) -> Self {
    Self{ data: Rc::clone(&registry.birthday ) }
  }
}

fn main() {
  let age = env::args().skip(1).next();
  
    let person = match age {
      Some( age ) => {
        let age = match str::parse(&age) {
          Ok(age) => age,
          Err( e ) => {
            println!( "Age is invalid. Using value 0" );
            println!( "Reason: {e}" );
            0
          }
        };
        Person {
          name: Rc::new(String::from("Jane")),
          age: Rc::new( age ),
          location: Rc::new( (0.0, 0.0) ),
          birthday: Rc::new( SystemTime::now() )
        }
      }
      None => { 
        println!("Run this passing 1 argument which is the age. Runninng with default value");
        Person { 
          name: Rc::new(String::from("Andrea")),
          age: Rc::new( 23 ),
          location: Rc::new( (3.14159265, 2.7182818) ),
          birthday: Rc::new( SystemTime::now() )
        }
      }
    };

  if is_old_enough_with_name.apply( &person ){
    do_it.apply( &person );
  } else {
    dont_do_it.apply( &person );
  }
}


fn is_old_enough_with_name( name: Data<String>, age: Data<u8> ) -> bool {
  if *age.data > 18 {
    println!("{} is old enough", name.data );
    true
  } else {
    println!("{} is not old enough", name.data );
    false
  }
}

fn do_it( location: Data<(f64, f64)> ){
  println!("Going to {:?}", location.data );
}

fn dont_do_it( location: Data<(f64, f64)>, age: Data<u8>){
  println!("Cannot go to {:?}, because age is only {}", location.data, age.data)
}