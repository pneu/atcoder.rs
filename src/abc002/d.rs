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
    assert_eq!(calc(5, &vec![(1, 2), (2, 3), (1, 3)]), 3);
    assert_eq!(calc(5, &vec![(1, 2), (2, 3), (3, 4)]), 2);
    assert_eq!(
        calc(
            7,
            &vec![
                (1, 2),
                (1, 3),
                (2, 3),
                (4, 5),
                (4, 6),
                (4, 7),
                (5, 6),
                (5, 7),
                (6, 7)
            ]
        ),
        4
    );
    assert_eq!(calc(12, &vec![]), 1);
}
#[allow(dead_code)]
pub fn main() {
    let v = reads::<u32>();
    let n = v[0];
    let m = v[1];
    let mut v: Vec<(u32, u32)> = Vec::new();
    for _ in 0..m {
        let r = reads::<u32>();
        v.push((r[0], r[1]));
    }
    let r = calc(n, &v);
    println!("{}", r);
}

#[allow(dead_code)]
fn calc(n: u32, v: &Vec<(u32, u32)>) -> u32 {
    // print_debug_v(v);
    let t = ph1(n, v);
    // print_debug_m(t);
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if t[i as usize][j as usize] == 1 {
                // わからん
            }
        }
    }
    0
}
#[allow(dead_code)]
fn ph1(n: u32, v: &Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    let mut t: Vec<Vec<u32>> = vec![vec![0u32; n as usize]; n as usize];
    for r in v {
        let r0 = (r.0 - 1) as usize;
        let r1 = (r.1 - 1) as usize;
        t[r0][r1] = 1;
        t[r1][r0] = 1;
    }
    t
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
