#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};
use std::collections::HashMap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut aa = a.clone().into_iter().collect::<HashSet<_>>();
    let mut aa = aa.into_iter().collect::<Vec<_>>();
    let mut map = HashMap::new();
    aa.sort();

    for i in 0..n {
        //let tmp = upper_bound(&aa, a[i]);
        //println!("{}", tmp);
        let ans = aa.len() - upper_bound(&aa, a[i]);
        *map.entry(ans).or_insert(0) += 1;
    }

    for i in 0..n {
        let ret = map.get(&i).unwrap_or(&0);
        println!("{}", ret);
    }
}

fn upper_bound<T: PartialOrd>(array: &[T], k: T) -> usize {
    let mut ok = array.len() as i64;
    let mut ng = -1;
    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if array[mid as usize] <= k {
            ng = mid;
        } else {
            ok = mid;
        }
    }
    ok as usize
}

pub fn lower_bound<T: PartialOrd>(array: &[T], k: T) -> usize {
    let mut ok = array.len() as i64;
    let mut ng = -1;
    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if array[mid as usize] < k {
            ng = mid;
        } else {
            ok = mid;
        }
    }
    ok as usize
}
