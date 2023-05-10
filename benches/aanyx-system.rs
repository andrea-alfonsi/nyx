use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use aanyx::system::{System, FromRegistry};


struct Name { name: String }
impl FromRegistry<()> for Name {
  fn from_registry( _: &() ) -> Name {
    black_box( Self { name: "Andrea".to_string() })
  }
}

struct Age{ age: u8 }
impl FromRegistry<()> for Age {
  fn from_registry( _: &() ) -> Age {
    black_box(Age { age: 7 })
  }
}

fn handler( name: Name, age: Age ) -> String { format!("Hello {:?}. You have {:?} years", name.name, age.age) }

fn bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("Compare System and Native calls");
  for i in [20u64, 40, 60, 80, 100, 120, 140, 160, 180, 200].iter() {
      group.bench_with_input(BenchmarkId::new("System", i), i, 
          |b, _i| b.iter(|| for _ in 0..*i { handler.apply(  &() ); } ));
      group.bench_with_input(BenchmarkId::new("Native", i), i, 
          |b, _i| b.iter(|| for _ in 0..*i { handler( black_box(Name { name : "Andrea".to_string() }), black_box( Age {age : 7 } ) ); }));
  }
  group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
