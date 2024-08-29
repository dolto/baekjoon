use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let result = big_add(input);

    stdout()
        .write_all(format!("{result}\n").as_bytes())
        .unwrap();

    // 'temp: for i in -100..=100 {
    //     for j in -100..=100 {
    //         let expect = i + j;
    //         let print = big_add(format!("{} {}", i, j)).parse().unwrap();
    //         if expect != print {
    //             println!("error: {expect}, {i} + {j} != result: {print}");
    //             break 'temp;
    //         }
    //     }
    // }
}

fn big_add(input: String) -> String {
    let mut input = input.split_ascii_whitespace();

    let mut left = input
        .next()
        .unwrap()
        .chars()
        .rev()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    let mut right = input
        .next()
        .unwrap()
        .chars()
        .rev()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let mut l_m = if left[left.len() - 1] == "-" {
        left.pop();
        true
    } else {
        false
    };
    let mut r_m = if right[right.len() - 1] == "-" {
        right.pop();
        true
    } else {
        false
    };

    // right >= left 를 확실하게 해야함
    if left.len() > right.len() {
        let temp = right;
        right = left;
        left = temp;

        let temp = r_m;
        r_m = l_m;
        l_m = temp;
    } else if left.len() == right.len() {
        for (index, l) in left.iter().enumerate().rev() {
            if l > &right[index] {
                let temp = right;
                right = left;
                left = temp;

                let temp = r_m;
                r_m = l_m;
                l_m = temp;
                break;
            } else if l == &right[index] {
                continue;
            } else {
                break;
            }
        }
    }

    if l_m == r_m {
        // left + right
        for index in 0..right.len() {
            let l = left
                .get(index)
                .unwrap_or(&"0".to_owned())
                .parse::<i32>()
                .unwrap();
            let r = right[index].parse::<i32>().unwrap();
            let res = r + l;
            if res > 9 {
                if let Some(right) = right.get_mut(index + 1) {
                    *right = (right.parse::<i32>().unwrap() + 1).to_string();
                } else {
                    right.push("1".to_string());
                }
            }
            right[index] = (res % 10).to_string();
        }
    } else {
        // left - right
        for index in 0..right.len() {
            let l = left
                .get(index)
                .unwrap_or(&"0".to_owned())
                .parse::<i32>()
                .unwrap();
            let r = right[index].parse::<i32>().unwrap();
            let mut res = r - l;
            // println!("before: {:?}", right);
            if res < 0 {
                if let Some(r) = right.get_mut(index + 1) {
                    *r = (r.parse::<i32>().unwrap() - 1).to_string();
                    res += 10;
                } else {
                    res = i32::abs(res);
                    r_m = !r_m;
                }
            }
            right[index] = res.to_string();
            // println!("after: {:?}", right);
        }
    }
    while right[right.len() - 1] == "0" && right.len() > 1 {
        right.pop();
    }
    if r_m && (right.len() != 1 || right[0] != "0") {
        right.push("-".to_string());
    }

    let result = right.into_iter().rev().collect::<String>();
    result
}
