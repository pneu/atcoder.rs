#[allow(dead_code)]
fn read<T>() -> T
where
    T: std::str::FromStr,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn main() {
    let m = read::<i32>();
    let vv = calc(m);
    println!("{}", vv);
}

fn calc(m: i32) -> String {
    let vv = if m < 100 {
        0
    } else if m <= 5000 {
        m * 10
    } else if m <= 30000 {
        m + 50000
    } else if m <= 70000 {
        (m - 30000) / 5 + 80000
    } else {
        /* if 70000 < m */
        89000
    };
    format!("{:>02}", (vv / 1000))
}

#[test]
fn test() {
    assert_eq!("65", calc(15000));
    assert_eq!("89", calc(75000));
    assert_eq!("02", calc(200));
}
