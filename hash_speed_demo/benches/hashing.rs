use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq)]
struct BadHashable {
    x: u32,
}

impl BadHashable {
    fn new(x: u32) -> BadHashable {
        BadHashable { x }
    }
}

impl Hash for BadHashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(8);
    }
}

#[derive(Eq, PartialEq)]
struct GoodHashable {
    x: u32,
}

impl GoodHashable {
    fn new(x: u32) -> GoodHashable {
        GoodHashable { x }
    }
}

impl Hash for GoodHashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x);
    }
}

fn insert_good_hashables_1k(bench: &mut Bencher) {
    bench.iter(|| {
        let mut myset = HashSet::new();
        for x in 0..1000 {
            myset.insert(GoodHashable::new(x));
        }
    })
}

fn insert_bad_hashables_1k(bench: &mut Bencher) {
    bench.iter(|| {
        let mut myset = HashSet::new();
        for x in 0..1000 {
            myset.insert(BadHashable::new(x));
        }
    })
}

fn insert_good_hashables_4k(bench: &mut Bencher) {
    bench.iter(|| {
        let mut myset = HashSet::new();
        for x in 0..4000 {
            myset.insert(GoodHashable::new(x));
        }
    })
}

fn insert_bad_hashables_4k(bench: &mut Bencher) {
    bench.iter(|| {
        let mut myset = HashSet::new();
        for x in 0..4000 {
            myset.insert(BadHashable::new(x));
        }
    })
}

benchmark_group!(
    benches,
    insert_good_hashables_1k,
    insert_bad_hashables_1k,
    insert_good_hashables_4k,
    insert_bad_hashables_4k,
);
benchmark_main!(benches);
