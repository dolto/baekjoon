use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let count = input.next().unwrap() as usize;
    let mut floor = input.rev().collect::<Vec<i32>>();
    // 마지막까지 모든 경우의 수를 봐야 하기때문에 더미데이터를 추가
    floor.push(0);
    floor.push(0);
    floor.push(0);
    floor.push(0);
    let mut log = Vec::with_capacity(count + 1);
    log.push(floor[0]);
    for i in 0..count + 3 {
        find_max(&mut log, &floor, i);
        // println!("{:?}", log);
    }

    stdout()
        .write_all(format!("{}", log[count + 3]).as_bytes())
        .unwrap();
}

fn find_max(log: &mut Vec<i32>, floor: &Vec<i32>, start: usize) {
    // todo: 현재지점(start)에서 +1로 가는 모든 경우의 수중 가장 큰 값을 log에 넣음
    // start-1 + [+1]
    // start + [+1]
    // 문제점: 연속된 3개의 계단을 오르면 안된다는 제약은 어떻게 체크할 것인가?
    // 해결법: 연속된 3개의 계단을 오르지 않는 모든 경우의 수를 확인
    // start-3 + [-2] + [0] + [+1] // 연속되면 3칸
    // start-3 + [-1] + [+1]
    // start-2 + [-1] + [+1]
    // start-2 + [0] + [+1] // 연계시 3칸
    // start-1 + [+1]
    // start + [+1] // 연계시 3칸

    if start != 0 {
        let mut max = 0;
        if start > 2 {
            max = log[start - 3] + floor[start - 1] + floor[start + 1];
        }
        if start > 1 {
            max = max.max(log[start - 2] + floor[start - 1] + floor[start + 1]);
        }
        max = max.max(log[start - 1] + floor[start + 1]);
        log.push(max);
    } else {
        log.push(log[start]);
    }
}
