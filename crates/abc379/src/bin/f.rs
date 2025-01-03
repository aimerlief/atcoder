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
    max,
    Ordering
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
        n: usize,
        q: usize,
        h: [usize; n],
    }

    let mut queries = vec![Vec::new(); n];
    for i in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let l = l - 1;
        let r = r - 1;
        queries[l].push((r, i));
    }

    let mut ft = FenwickTree::new(n, 0isize);
    let mut stc = Vec::new();
    let mut ans = vec![0usize; q];

    for l in (0..n).rev() {
        for &(r, idx) in &queries[l] {
            let res = ft.sum(0..n) - ft.sum(0..r + 1);
            ans[idx] = res as usize;
        }

        while let Some(&last) = stc.last() {
            if h[last] < h[l] {
                ft.add(last, -1);
                stc.pop();
            } else {
                break;
            }
        }
        stc.push(l);
        ft.add(l, 1);
    }

    for a in ans {
        println!("{}", a);
    }
}
