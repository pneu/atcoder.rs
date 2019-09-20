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
    assert_eq!(calc("chokudai"), "chkd");
    assert_eq!(calc("okanemochi"), "knmch");
    assert_eq!(calc("aoki"), "k");
    assert_eq!(calc("mazushii"), "mzsh");
}
#[allow(dead_code)]
pub fn main() {
    let s = read_str();
    let r = calc(&s);
    println!("{}", r);
}

#[allow(dead_code)]
fn calc(s: &str) -> String {
    let cs = s
        .chars()
        .filter(|c| *c != 'a' && *c != 'i' && *c != 'u' && *c != 'e' && *c != 'o')
        .collect::<String>();
    cs
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
