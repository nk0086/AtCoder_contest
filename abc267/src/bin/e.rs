#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        uv: [(usize, usize); m]
    }
}
