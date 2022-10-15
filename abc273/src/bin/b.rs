#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        mut x: u128,
        k: usize,
    }

    let mut Mod = 1;
    for i in 0..k {
        x = x / Mod;
        let tmp = x % 10;
        if tmp < 5 {
            x = x / 10 * 10;
        } else {
            x = (x / 10 + 1) * 10;
        }

        x *= Mod;
        Mod *= 10;
    }

    println!("{}", x);
}
