use std::{
    collections::VecDeque,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut left = input.next().unwrap().chars().collect::<VecDeque<char>>();
    let mut right = input.next().unwrap().chars().collect::<VecDeque<char>>();

    let mut output = String::new();

    let mut up = 0;
    let mut l_m = if left[0] == '-' {
        left.pop_front();
        true
    } else {
        false
    };
    let mut r_m = if right[0] == '-' {
        right.pop_front();
        true
    } else {
        false
    };

    if left.len() < right.len() {
        let temp = left;
        left = right;
        right = temp;

        let temp = l_m;
        l_m = r_m;
        r_m = temp;
    }
    while !left.is_empty() || !right.is_empty() {
        let l = left
            .pop_back()
            .unwrap_or('0')
            .to_string()
            .parse::<i32>()
            .unwrap();
        let r = right
            .pop_back()
            .unwrap_or('0')
            .to_string()
            .parse::<i32>()
            .unwrap();
        if l_m == r_m {
            let res = l + r + up;
            if res > 9 {
                up = 1;
            } else {
                up = 0;
            }
            output = format!("{}", res % 10) + output.as_str();
        } else {
            let mut res = l - r - up;
            if res < 0 {
                res += 10;
                up = 1;
            } else {
                up = 0;
            }
            output = format!("{}", res) + output.as_str();
        }
    }

    if l_m == r_m {
        if up == 1 {
            output = format!("1") + output.as_str();
        } else {
            let mut o = output
                .chars()
                .next()
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap();
            while o == 0 && output.len() > 1 {
                output = output[1..].to_string();
                o = output
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
            }
        }
        if l_m && output.chars().next().unwrap() != '-' && output != "0" {
            output = "-".to_string() + output.as_str();
        }
    } else {
        let mut o = output
            .chars()
            .next()
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        if up == 1 {
            output = output[1..].to_string();
            o -= 10;
            output = o.to_string() + output.as_str();
        }
        while o == 0 && output.len() > 1 {
            output = output[1..].to_string();
            o = output
                .chars()
                .next()
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap();
        }
        if l_m && output.chars().next().unwrap() != '-' && output != "0" {
            output = "-".to_string() + output.as_str();
        }
    }

    stdout().write_all(output.as_bytes()).unwrap();
}
