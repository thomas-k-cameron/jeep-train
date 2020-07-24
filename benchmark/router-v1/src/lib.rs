#![feature(test)]
extern crate test;
use test::Bencher;

use jeep_train_macro::router;
use jeep_train_prelude::*;
use actix_router::{Router, RouterBuilder, Path, ResourceInfo, IntoPattern};
use dataset::{
    arango_keys,
    benchmark_dataset,
    ReturnType
};

#[bench]
fn router_actix(b: &mut test::Bencher) {
    let vec_s = benchmark_dataset();

    let mut build: RouterBuilder<Path<String>, ()> = Router::build();
    for s in vec_s.iter() {
        let p = Path::new(s.to_owned());
        build.path(s.to_owned(), p);
    }

    let finished = build.finish();

    let mut vec = {
        let bench_path = dataset::benchmark_dataset();
        let mut vec = vec![];
        for i in bench_path.iter() {
            vec.push(Path::new(i.to_owned()))
        }
        vec
    };


    b.iter(|| {
        for b in vec.iter_mut() {
            finished.recognize(b).is_none();
        }
    });
}


bench_macro::main!();

#[inline]
fn benchmark_func(x: Conn) -> bool {
    return true
}

#[bench]
fn router_jeep_train(b: &mut test::Bencher) {
    let mut bench_path = dataset::benchmark_dataset();
    let mut v = Vec::with_capacity(bench_path.len());
    bench_path.iter().for_each(|i| {
        use std::sync::Arc;
        let mut conn = BearConnection::default();
        conn.path = i.to_owned();
        v.push(Arc::new(BearConnection::default()))
    });
    b.iter(|| {
        for i in v.iter() {
            BENCHMARK_ROUTER(i.clone());
        }
    });
}
