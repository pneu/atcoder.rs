use std::fmt::Debug;
use std::io::stdin;
use std::str::FromStr;

#[allow(dead_code)]
fn read_str() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
#[allow(dead_code)]
fn reads<T: FromStr>() -> Vec<T> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>()
}

#[test]
fn name() {
    assert_eq!(calc(&vec![1, 0, 3, 0, 2, 5]), 5.0);
    assert_eq!(calc(&vec![-1, -2, 3, 4, 5, 6]), 2.0);
    assert_eq!(calc(&vec![298, 520, 903, 520, 4, 663]), 43257.5);
}
#[allow(dead_code)]
pub fn main() {
    let v = reads();
    let r = calc(&v);
    println!("{}", r);
}

#[allow(dead_code)]
fn calc(v: &Vec<i32>) -> f64 {
    let xa = v[0];
    let ya = v[1];
    let xb = v[2];
    let yb = v[3];
    let xc = v[4];
    let yc = v[5];

    let a = xa * yb + xb * yc + xc * ya - (xa * yc + xc * yb + xb * ya);
    // dround((xb - xa) * (yc - ya) - (yb - ya) * (xc - xa)).abs()
    ((a as f64) / 2f64).abs()
}

#[allow(dead_code)]
fn print_debug_i<I>(i: I)
where
    I: Iterator,
    <I as Iterator>::Item: Debug,
{
    for (i, x) in i.enumerate() {
        println!("{} {:?}", i, x);
    }
}
