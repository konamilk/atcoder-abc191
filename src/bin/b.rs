use proconio::input;

fn main() {
    input!{
        n: i32,
        x: i32,
        a: [i32; n]
    }

    let mut ans = String::from("");

    for integer in a {
        if integer == x {continue}

        ans.push_str(&(integer.to_string()));
        ans.push_str(" ")
    }

    if ans.len() > 0 {
        ans.truncate(ans.len() - 1)
    }

    println!("{}", ans)
}
