#[allow(dead_code)]
fn print_by_chars(s: &str) {
    // chars: 文字のイテレータを返す。Chars は文字列スライスのイテレータで、std::iter::Iterator を実装する。
    let chars_it = s.chars();
    // イテレータを評価してコレクションを返す。
    let cs: Vec<char> = chars_it.collect();
    println!("{:?}", cs);
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse::<T>().ok().unwrap()
}

#[allow(dead_code)]
pub fn main() {
    let h1 = read::<i32>();
    let h2 = read::<i32>();

    println!("{}", h1 - h2);
}
