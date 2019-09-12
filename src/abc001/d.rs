#[allow(dead_code)]
fn read_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
pub fn main() {
    let n = read::<u32>();
    let mut v: Vec<String> = Vec::new();
    for _ in 0..n {
        let times = read_str();
        v.push(times);
    }
    let r = calc(&v);
    for x in r.iter() {
        println!("{}", x);
    }
}

fn split(s: &str) -> Vec<String> {
    s // "hhmm-iinn"
        .split('-') // ["hhmm", "iinn"]
        .map(|x| x.to_string())
        .collect()
}

fn calc(v: &Vec<String>) -> Vec<String> {
    let mut t: [i32; 60 * 24 + 1] = [0; 60 * 24 + 1];
    for x in v.iter() {
        let s = split(x);
        let b = clip_begin(to_min(&s[0]));
        t[b as usize] += 1;
        let e = clip_end(to_min(&s[1]));
        t[e as usize] -= 1;
    }
    // simulate
    let mut r: Vec<(u32, u32)> = Vec::new();
    let mut s: u32 = 0;
    let mut f: bool = false;
    for x in 0..60 * 24 + 1 {
        if 0 < x {
            t[x] += t[x - 1];
        }

        if !f && (t[x] > 0) {
            s = x as u32;
            f = true;
        }
        if f && (t[x] == 0) {
            let t: (u32, u32) = (s, x as u32);
            r.push(t);
            f = false;
        }
    }

    let rs: Vec<String> = r
        .iter()
        .map(|x| {
            let h0 = x.0 / 60;
            let m0 = x.0 % 60;
            let h1 = x.1 / 60;
            let m1 = x.1 % 60;
            format!("{:02}{:02}-{:02}{:02}", h0, m0, h1, m1)
        })
        .collect();
    rs
}
fn to_min(hm: &String) -> u32 {
    let h: u32 = hm[0..2].parse().unwrap();
    let m: u32 = hm[2..4].parse().unwrap();
    h * 60 + m
}
fn clip_begin(n: u32) -> u32 {
    ((n / 5) as u32) * 5
}
fn clip_end(n: u32) -> u32 {
    (((n + 4) / 5) as u32) * 5
}

#[allow(dead_code)]
fn print_debug_i<I>(i: I)
where
    I: Iterator,
    <I as Iterator>::Item: std::fmt::Debug,
{
    for (i, x) in i.enumerate() {
        println!("{} {:?}", i, x);
    }
}

#[test]
fn name() {
    assert_eq!(calc_(&vec!["0000-2400"]), vec!["0000-2400"]);
    assert_eq!(
        calc_(&vec!["1148-1210", "1323-1401", "1106-1123", "1129-1203",]),
        vec!["1105-1210", "1320-1405"]
    );
    assert_eq!(
        calc_(&vec![
            "1157-1306",
            "1159-1307",
            "1158-1259",
            "1230-1240",
            "1157-1306",
            "1315-1317",
        ]),
        vec!["1155-1310", "1315-1320"]
    );
}
#[allow(dead_code)]
fn calc_(v: &Vec<&str>) -> Vec<String> {
    let xs: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    calc(&xs)
}
