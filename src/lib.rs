#![feature(test)]

extern crate rand;
extern crate test;

pub trait Simple {
    fn method1(&self, x: u32) -> u32;
}

#[derive(Copy, Clone)]
pub struct T1 {
    v: u32,
}

impl Simple for T1 {
    fn method1(&self, x: u32) -> u32 {
        self.v + x + 5
    }
}

#[derive(Copy, Clone)]
pub struct T2 {
    v: u32,
}

impl Simple for T2 {
    fn method1(&self, x: u32) -> u32 {
        self.v + x + 6
    }
}

#[derive(Copy, Clone)]
pub struct T3 {
    v: u32,
}

impl Simple for T3 {
    fn method1(&self, x: u32) -> u32 {
        self.v + x + 7
    }
}

pub fn ret_simple(s: u32, v: u32) -> Box<Simple> {
    match s {
        0 => Box::new(T1{ v }),
        1 => Box::new(T2{ v }),
        _ => Box::new(T3{ v }),
    }
}

use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use test::Bencher;

#[bench]
fn bench_ret_simple(b: &mut Bencher) {
    b.iter(|| {
        let between = Range::new(0, 3);
        let mut rng = rand::thread_rng();
        let v = rng.gen();
        let b = ret_simple(between.ind_sample(&mut rng), v);
        b.method1(0)
    });
}

union DataUnion {
    t1: T1,
    t2: T2,
    t3: T3,
}

struct OptRet {
    data: DataUnion,
    method1: fn(s: &DataUnion, x: u32) -> u32,
}

use std::mem;

fn ret_simple_opt(s: u32, v: u32) -> OptRet {
    match s {
        0 => {
            let ptr: fn(&T1, u32) -> u32 = <T1 as Simple>::method1;
            OptRet {
                data: DataUnion {
                    t1: T1{ v }
                },
                method1: unsafe { mem::transmute(ptr) }
            }
        },
        1 => {
            let ptr: fn(&T2, u32) -> u32 = <T2 as Simple>::method1;
            OptRet {
                data: DataUnion {
                    t2: T2{ v }
                },
                method1: unsafe { mem::transmute(ptr) }
            }
        },
        _ => {
            let ptr: fn(&T3, u32) -> u32 = <T3 as Simple>::method1;
            OptRet {
                data: DataUnion {
                    t3: T3{ v }
                },
                method1: unsafe { mem::transmute(ptr) }
            }
        }
    }
}

#[bench]
fn bench_ret_simple_opt(b: &mut Bencher) {
    b.iter(|| {
        let between = Range::new(0, 3);
        let mut rng = rand::thread_rng();
        let v = rng.gen();
        let b = ret_simple_opt(between.ind_sample(&mut rng), v);
        (b.method1)(&b.data, 0)
    });
}

#[test]
fn test_ret_simple_opt() {
    let b = ret_simple_opt(0, 10);
    assert_eq!((b.method1)(&b.data, 5), 20);
    let b = ret_simple_opt(1, 10);
    assert_eq!((b.method1)(&b.data, 5), 21);
    let b = ret_simple_opt(2, 10);
    assert_eq!((b.method1)(&b.data, 5), 22);
}
