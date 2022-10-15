#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    // ans += a[i]
    let mut ans = 0;
    for i in 0..n {
        ans += a[i];
    }
    println!("{}", ans);
}
