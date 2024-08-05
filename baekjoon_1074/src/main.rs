use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|n| n.parse::<usize>());

    let n = input.next().unwrap();
    let n = 2 << n;
    let y = input.next().unwrap();
    let x = input.next().unwrap();

    let mut count = 0;
    let mut goal = 0;
    set_map(0, n, 0, n, &x, &y, &mut goal, &mut count);

    stdout().write_all(format!("{}", goal).as_bytes()).unwrap();
}

fn set_map(
    sx: usize,
    ex: usize,
    sy: usize,
    ey: usize,
    gx: &usize,
    gy: &usize,
    goal: &mut usize,
    count: &mut usize,
) {
    if ex - sx > 2 {
        let mx = (ex + sx) / 2;
        let my = (ey + sy) / 2;
        let half = (ex - mx) * (ey - my);
        if sx <= *gx && mx > *gx && sy <= *gy && my > *gy {
            // println!("1번 구역 {}", *count);
            set_map(sx, mx, sy, my, gx, gy, goal, count);
        } else if mx <= *gx && ex > *gx && sy <= *gy && my > *gy {
            *count += half;
            set_map(mx, ex, sy, my, gx, gy, goal, count);
        } else if sx <= *gx && mx > *gx {
            // println!("3번 구역 {}", *count);
            *count += half * 2;
            set_map(sx, mx, my, ey, gx, gy, goal, count);
        } else {
            *count += half * 3;
            set_map(mx, ex, my, ey, gx, gy, goal, count);
        }
    } else {
        // println!("직접 계산 {}", *count);
        for y in sy..ey {
            for x in sx..ex {
                if x == *gx && y == *gy {
                    *goal = *count;
                } else {
                    *count += 1;
                }
            }
        }
    }
}
