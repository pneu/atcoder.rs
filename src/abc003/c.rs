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
    assert_eq!(calc(2, 2, &mut vec![1000, 1500]), 1000.000000f64);
    assert_eq!(calc(2, 1, &mut vec![1000, 1500]), 750f64);
    assert_eq!(
        calc(
            10,
            5,
            &mut vec![2604, 2281, 3204, 2264, 2200, 2650, 2229, 2461, 2439, 2211]
        ),
        2820.031250000f64
    );
}
#[allow(dead_code)]
pub fn main() {
    let nk = reads::<u32>();
    let mut r = reads::<u32>();
    let ans = calc(nk[0], nk[1], &mut r);
    println!("{}", ans);
}

#[allow(dead_code)]
fn calc(_n: u32, k: u32, r: &mut Vec<u32>) -> f64 {
    // 全組み合わせは莫大な数になる: P(n, k) = n! / (n - k)!
    // n = 100, k = 100 のとき、100! = O(10^157)
    //
    // 規則性を発見するため、小さい数で試してみる。
    // ghci > import qualified Data.List as L
    // ghci > perm movies = L.reverse $ L.sortBy (\(_, a) (_, b) -> compare a b) [(map ceiling xs, foldl (\x y -> (x + y) / 2.0) 0 xs) | xs <- L.permutations movies]
    // ghci > perm [1,2]
    // [([1,2],1.25),([2,1],1.0)]
    // ghci > perm [1,2,3]
    // [([1,2,3],2.125),([2,1,3],2.0),([1,3,2],1.875),([3,1,2],1.625),([2,3,1],1.5),([3,2,1],1.375)]    //
    // > perm [1, 10, 100, 1000, 10000]
    // [([1,10,100,1000,10000],5263.15625),...
    // > perm [10000, 10001, 10002, 10003, 10004]
    // [([10000,10001,10002,10003,10004],9690.5625),
    //
    // 上で試した結果から分かること:
    //   全部を選択する場合は、小さい順に足していくと最大値が得られる。(っぽい)
    // それと、足し合わせるものが大きい方が、最大値が大きくなるだろう。
    //
    // まとめると、次の方針で。
    // - N個のうち、大きい方からK個選択する。
    // - K個を小さい方から畳み込む。
    r.sort(); // ascending
    let ks = &r[(r.len() - (k as usize))..r.len()]; // n-k .. n
    let sum = ks.iter().fold(0f64, |acc, x| (acc + *x as f64) / 2f64);
    sum
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
