use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i64>());

    let hurt_finger = input.next().unwrap();
    let can_use = input.next().unwrap();

    // 다친 손가락 직전부터 시작
    let mut answer = hurt_finger - 1;

    if can_use <= 0 {
        write!(stdout(), "{}", answer).unwrap();
        return;
    }

    if hurt_finger > 1 && hurt_finger < 5 {
        let temp = can_use / 2;
        let left = can_use % 2;

        answer += temp * 8;
        // 정방향에서 시작함
        if left == 1 {
            answer += 10 - hurt_finger * 2;
        }
    } else {
        answer += can_use * 8;
    }

    write!(stdout(), "{}", answer).unwrap();

    stdout().flush().unwrap();
}
