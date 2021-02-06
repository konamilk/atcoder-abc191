use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h]
    }

    let mut choten = vec![vec![0;100];100];

    for i in 0..w {
        for j in 0..h{
            if s[j][i] == '#'{
                choten[i][j] += 1;
                choten[i+1][j] += 1;
                choten[i][j+1] += 1;
                choten[i+1][j+1] += 1;
            }
        }
    }

    let mut ans = 0;

    for i in 0..20{
        for j in 0..20{
            if choten[j][i] % 2  == 1{
                ans += 1
            }
        }
    }

    println!("{}", ans)
}
