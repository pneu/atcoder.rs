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
    read_str().parse().ok().unwrap()
}
#[allow(dead_code)]
fn reads<T: FromStr>() -> Vec<T> {
    read_str()
        .split_whitespace()
        .map(|x| x.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>()
}

#[test]
fn name() {
    assert_eq!(calc(6), 35000f64);
    assert_eq!(calc(91), 460000f64);
}
#[allow(dead_code)]
pub fn main() {
    let n = read::<u32>();
    let ans = calc(n);
    println!("{}", ans);
}

#[allow(dead_code)]
fn calc(n: u32) -> f64 {
    // 10000 * {(1/n)+(2/n)+ ... +(k/n)+ ... +(n/n)}
    let n = n as f64;
    let mut s = n * (1f64 + n) * 10000f64;
    s = s / (2f64 * n);
    s
}

#[allow(dead_code)]
fn print_debug_v<I>(v: I)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Debug,
{
    for (i, x) in v.into_iter().enumerate() {
        println!("[{}]: {:?}", i, x);
    }
}
#[allow(dead_code)]
fn print_debug_m<I>(m: I)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: IntoIterator,
    <<I as IntoIterator>::Item as IntoIterator>::Item: Debug,
{
    // let max = m.into_iter().map(|j|j.into_iter().count()).max();
    for (i, x) in m.into_iter().enumerate() {
        print!("[{}]: ", i);
        for (_, y) in x.into_iter().enumerate() {
            print!("{:?}, ", y);
        }
        println!("");
    }
}
