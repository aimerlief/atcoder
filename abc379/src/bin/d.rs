#[allow(unused_imports)]
use proconio::{
    input,
    fastout,
    marker::{Isize1,Usize1,Chars,Bytes}
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{
    VecDeque,
    LinkedList,
    HashMap,
    BTreeMap,
    HashSet,
    BTreeSet,
    BinaryHeap
};

#[allow(unused_imports)]
use std::cmp::{
    min,
    max
};

#[allow(unused_imports)]
use ac_library::{
    Dsu,
    // new(size: usize) -> Self
    // merge(&mut self, a: usize, b: usize) -> usize
    // same(&mut self, a: usize, b: usize) -> bool
    // leader(&mut self, a: usize) -> usize
    // size(&mut self, a: usize) -> usize
    // groups(&mut self) -> Vec<Vec<usize>>
    FenwickTree,
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    math,
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&mut self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree,
    Max
};

fn main() {
    input! {
        q: usize,
    }

    let mut plant_times = BTreeMap::new();
    let mut current_time: u64 = 0;
    let mut results = Vec::new();

    for _ in 0..q {
        input! {
            query: String,
        }

        match &query[..] {
            "1" => {
                *plant_times.entry(current_time).or_insert(0) += 1;
            }
            "2" => {
                input! {
                    t: u64,
                }
                current_time += t;
            }
            "3" => {
                input! {
                    h: u64,
                }
                if current_time >= h {
                    let threshold_time = current_time - h;
                    let mut keys_to_remove = Vec::new();
                    let mut harvested_count = 0;

                    for (&plant_time, &count) in plant_times.range(..=threshold_time) {
                        harvested_count += count;
                        keys_to_remove.push(plant_time);
                    }

                    for key in keys_to_remove {
                        plant_times.remove(&key);
                    }

                    results.push(harvested_count);
                } else {
                    results.push(0);
                }
            }
            _ => {}
        }
    }

    for ans in results {
        println!("{}", ans);
    }
}
