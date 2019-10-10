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
    assert_eq!(calc("ch@ku@ai", "choku@@i"), true);
    assert_eq!(calc("aoki", "@ok@"), false);
    assert_eq!(calc("arc", "abc"), false);

    assert_eq!(is_atcoder('a'), true);
    assert_eq!(is_atcoder('t'), true);
    assert_eq!(is_atcoder('r'), true);
    assert_eq!(is_atcoder('z'), false);
    assert_eq!(is_atcoder('@'), false);
}
#[allow(dead_code)]
pub fn main() {
    let s = read_str();
    let t = read_str();
    let ans = calc(&s, &t);
    println!("{}", if ans { "You can win" } else { "You will lose" });
}

#[allow(dead_code)]
fn calc(s: &str, t: &str) -> bool {
    // s.len() == t.len()
    s.chars()
        .zip(t.chars())
        .map(|cs| {
            if cs.0 == cs.1 {
                true
            } else {
                if cs.0 == '@' && cs.1 != '@' {
                    is_atcoder(cs.1)
                } else if cs.0 != '@' && cs.1 == '@' {
                    is_atcoder(cs.0)
                } else
                /* cs.0 != '@' && cs.0 != '@' && cs.0 != cs.1 */
                {
                    false
                }
            }
        })
        .all(|x| x)
}
#[allow(dead_code)]
fn is_atcoder(c: char) -> bool {
    c == 'a' || c == 't' || c == 'c' || c == 'o' || c == 'd' || c == 'e' || c == 'r'
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
