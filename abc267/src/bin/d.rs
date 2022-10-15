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

    let mut dp = vec![vec![-1 << 63; m + 2]; n + m + 10];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m + 1 {
            if dp[i][j] == -1 << 63 {
                continue;
            }

            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + a[i] * (j as i64 + 1));
        }
    }

    println!("{}", dp[n][m]);
}
