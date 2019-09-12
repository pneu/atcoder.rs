#[allow(dead_code)]
fn read<T>() -> T
where
    T: std::str::FromStr,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    parse(&s)
}
fn parse<T>(s: &String) -> T
where
    T: std::str::FromStr,
{
    s.trim().parse().ok().unwrap()
}
#[allow(dead_code)]
fn read_line<T, U>() -> (T, U)
where
    T: std::str::FromStr,
    U: std::str::FromStr,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let ss: Vec<&str> = s.trim().split(' ').collect();
    (parse(&ss[0].to_string()), parse(&ss[1].to_string()))
}
trait Round45 {
    fn round45_p(&self, n: i32) -> Self;
}
impl Round45 for f32 {
    fn round45_p(&self, n: i32) -> Self {
        let dec = 10_f32.powi(n);
        let dec_ = 10_f32.powi(n - 1);
        let r = ((self * dec + 5_f32) / 10_f32) as i32 as f32 / dec_;
        r
    }
}
impl Round45 for f64 {
    fn round45_p(&self, n: i32) -> Self {
        let dec = 10_f64.powi(n);
        let dec_ = 10_f64.powi(n - 1);
        let r = ((self * dec + 5_f64) / 10_f64) as i32 as f64 / dec_;
        r
    }
}
#[allow(dead_code)]
fn in_range_ex<T: std::cmp::PartialOrd>(x: T, begin: T, end: T) -> bool {
    if begin <= x && x < end {
        true
    } else {
        false
    }
}
#[allow(dead_code)]
fn in_range<T: std::cmp::PartialOrd>(x: T, start: T, last: T) -> bool {
    if start <= x && x <= last {
        true
    } else {
        false
    }
}

pub fn main() {
    let (deg, dis) = read_line::<i32, f32>();
    println!("{}", calc(deg, dis));
}

#[test]
fn test() {
    assert_eq!("W 5", calc(2750, 628_f32));
    assert_eq!("C 0", calc(161, 8_f32));
    assert_eq!("NNW 1", calc(3263, 15_f32));
    assert_eq!("SE 12", calc(1462, 1959_f32));
    assert_eq!("SSE 8", calc(1687, 1029_f32));
    assert_eq!("WSW 5", calc(2587, 644_f32));
    assert_eq!("NNE 3", calc(113, 201_f32));
    assert_eq!("SSW 1", calc(2048, 16_f32));

    assert_eq!(10_f32, 10.251_f32.round45_p(0));
    assert_eq!(10_f32, 10.251_f32.round45_p(1));
    assert_eq!(10.3_f32, 10.251_f32.round45_p(2));
    assert_eq!(10.25_f32, 10.251_f32.round45_p(3));
}

fn calc(deg: i32, dis: f32) -> String {
    let deg = deg * 10;
    let dg = if in_range_ex(deg, 1125, 3375) {
        "NNE"
    } else if in_range_ex(deg, 3375, 5625) {
        "NE"
    } else if in_range_ex(deg, 5625, 7875) {
        "ENE"
    } else if in_range_ex(deg, 7875, 10125) {
        "E"
    } else if in_range_ex(deg, 10125, 12375) {
        "ESE"
    } else if in_range_ex(deg, 12375, 14625) {
        "SE"
    } else if in_range_ex(deg, 14625, 16875) {
        "SSE"
    } else if in_range_ex(deg, 16875, 19125) {
        "S"
    } else if in_range_ex(deg, 19125, 21375) {
        "SSW"
    } else if in_range_ex(deg, 21375, 23625) {
        "SW"
    } else if in_range_ex(deg, 23625, 25875) {
        "WSW"
    } else if in_range_ex(deg, 25875, 28125) {
        "W"
    } else if in_range_ex(deg, 28125, 30375) {
        "WNW"
    } else if in_range_ex(deg, 30375, 32625) {
        "NW"
    } else if in_range_ex(deg, 32625, 34875) {
        "NNW"
    } else {
        "N"
    };

    let dis = ((dis / 60.0).round45_p(2) * 10.0) as i32;
    let ds = if in_range(dis, 0, 2) {
        0
    } else if in_range(dis, 3, 15) {
        1
    } else if in_range(dis, 16, 33) {
        2
    } else if in_range(dis, 34, 54) {
        3
    } else if in_range(dis, 55, 79) {
        4
    } else if in_range(dis, 80, 107) {
        5
    } else if in_range(dis, 108, 138) {
        6
    } else if in_range(dis, 139, 171) {
        7
    } else if in_range(dis, 172, 207) {
        8
    } else if in_range(dis, 208, 244) {
        9
    } else if in_range(dis, 245, 284) {
        10
    } else if in_range(dis, 285, 326) {
        11
    } else {
        12
    };
    let dg = if ds == 0 { "C" } else { dg };
    format!("{} {}", dg.to_string(), ds.to_string())
}
