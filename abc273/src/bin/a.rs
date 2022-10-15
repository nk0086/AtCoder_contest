#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 1;
    //f(k) = k * f(k - 1)
    //f(0) = 1

    for i in 0..n {
        ans *= i + 1;
    }

    println!("{}", ans);
}
