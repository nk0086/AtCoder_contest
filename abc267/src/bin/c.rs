#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n]
    }

    let mut tmp = 0;
    let mut diff = 0;
    for i in 0..m {
        tmp += (i as i64 + 1) * a[i];
        diff += a[i];
    }

    let mut ans = tmp;
    for i in m..n {
        tmp -= diff;
        tmp += m as i64 * a[i];
        diff -= a[i - m];
        diff += a[i];
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
