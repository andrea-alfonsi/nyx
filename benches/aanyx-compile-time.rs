use std::{collections::HashMap, rc::Rc, ops::Add};

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use aanyx::compile_time::{Service, from_registry::hashmap};

fn no_args() -> i64 {
  black_box( 1 ) * 65
}

fn add_one( n: Rc::<i64>) -> i64 {
  n.add( black_box(1) )
}


fn criterion_benchmark(c: &mut Criterion){
  let service_no_args = Service::new( Box::new( no_args ));
  let service_add_one = Service::new( Box::new(add_one) );
  let map: hashmap::HashMapType = { 
    let mut map = HashMap::new();
    map.insert( std::any::TypeId::of::<Rc<i64>>(), Box::new(Rc::new( 64i64 )) as Box<dyn std::any::Any>);
    map
  };

  let mut group_no_args = c.benchmark_group("Service vs fn [no_args]");
  for n_calls in [10, 100, 1000, 10_000].iter() {
    group_no_args.bench_with_input( BenchmarkId::new("Service", n_calls), n_calls, 
    |b, n| {
      b.iter(||{ for _ in 0..*n { service_no_args.call( &() ); black_box( () ); }})
    });

    group_no_args.bench_with_input( BenchmarkId::new("fn", n_calls), n_calls, 
    |b, n| {
      b.iter(||{ for _ in 0..*n { no_args(); black_box( () ); }})
    });
  }
  group_no_args.finish();

  let mut group_no_args = c.benchmark_group("Service vs fn [Rc<i64>]");
  for n_calls in [10, 100, 1000, 10_000].iter() {
    group_no_args.bench_with_input( BenchmarkId::new("Service", n_calls), n_calls, 
    |b, n| {
      b.iter(||{ for _ in 0..*n { service_add_one.call( &map ); black_box( () ); }})
    });

    let num = Rc::new( 64i64 );
    group_no_args.bench_with_input( BenchmarkId::new("fn", n_calls), n_calls, 
    |b, n| {
      b.iter(||{ for _ in 0..*n { add_one( Rc::clone(&num)); black_box( () ); }})
    });
  }
  group_no_args.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);