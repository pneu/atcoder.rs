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
    assert_eq!(calc(&vec![10, 11]), 11);
    assert_eq!(calc(&vec![100000000, 10000000]), 100000000);
}
#[allow(dead_code)]
pub fn main() {
    let xs = reads::<u32>();
    let r = calc(&xs);
    println!("{}", r);
}

#[allow(dead_code)]
fn calc(v: &Vec<u32>) -> u32 {
    let m = v.iter().max();
    *m.unwrap()
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
