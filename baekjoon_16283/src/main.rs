use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .trim()
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let (a, b, n, w) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    let mut found = false;
    let mut sheep = 0;
    let mut goat = 0;

    for s in 1..n {
        let g = n - s;
        if a * s + b * g == w {
            if found {
                stdout().write_all("-1".as_bytes()).unwrap();
                return;
            }
            found = true;
            sheep = s;
            goat = g;
        }
    }

    if found {
        stdout()
            .write_all(format!("{sheep} {goat}").as_bytes())
            .unwrap();
    } else {
        stdout().write_all("-1".as_bytes()).unwrap();
    }

    // 여러개의 해중 하나만 구해도 된다면 다음과 같은 식을 사용해도 된다.
    // let mut sheep = n / 2;
    // let mut goat = n / 2;

    // if n > sheep + goat {
    //     sheep += 1;
    // }

    // let total = sheep * a + goat * b;
    // if total != w {
    //     let mine = total - w;
    //     if a > b {
    //         let c = mine / (a - b);
    //         sheep -= c;
    //         goat += c;
    //     } else if a < b {
    //         let c = mine / (b - a);
    //         sheep += c;
    //         goat -= c;
    //     }
    // }

    // if sheep == 0 || goat == 0 || total != w {
    //     stdout().write_all("-1".as_bytes()).unwrap();
    // } else {
    //     stdout()
    //         .write_all(format!("{sheep} {goat}").as_bytes())
    //         .unwrap();
    // }
}
