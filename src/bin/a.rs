use proconio::input;

fn main() {
    input!{
    v: i32,
    t: i32,
    s: i32,
    d: i32

    }

    if v * t <= d && d <= s* v{
        println!("No")
    }
    else {
        println!("Yes")
    }
}
