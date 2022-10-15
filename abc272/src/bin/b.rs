#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    }

    let mut x = vec![vec![]; m];

    for i in 0..m {
        input! {
            k: usize,
            xx: [usize; k],
        }

        for j in 0..k {
            x[i].push(xx[j]);
        }
    }

    let mut ans = true;
    for i in 0..n {
        let mut check = vec![];
        for j in 0..m {
            if x[j].contains(&(i + 1)) {
                check.append(&mut x[j]);
            }
        }

        println!("{:?}", check);
        let check = check.iter().collect::<HashSet<_>>();
        if check.len() != m {
            ans = false;
            break;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
