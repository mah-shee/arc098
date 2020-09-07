#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cur = 0;
    for i in s.iter() {
        if *i == 'E' {
            cur += 1;
        }
    }
    let mut ans = cur;
    for i in 0..n {
        if s[i] == 'E' {
            cur -= 1;
        } else {
            cur += 1;
        }
        ans = std::cmp::min(ans, cur);
    }
    println!("{}", ans);
}
