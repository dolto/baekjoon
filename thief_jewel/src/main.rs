// 보석점에 보석은 N개, 각 보석은 무게와 가치를 갖고 있다.
// 가방이 하나가 아니라 같은 용량C의 가방 여러개로 구성되어있다.

// 아... 가방에는 최대 하나의 보석만 들어감
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, stdout, BufReader, Read, Write},
};

fn main() {
    let mut temp = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut temp).unwrap();
    let mut input = temp.split_ascii_whitespace();

    let jewels_count = input.next().unwrap().trim().parse().unwrap();
    let bag_count = input.next().unwrap().trim().parse().unwrap();

    // 최소힙으로 언제나 용량이 가장 적은 것을 꺼낼 수 있음
    let mut bag_heap = BinaryHeap::with_capacity(bag_count);
    let mut jewels = Vec::with_capacity(jewels_count);
    // 최대힙으로 언제나 가치가 가장 높은 보석을 꺼낼 수 있음
    let mut jewels_value_heap = BinaryHeap::with_capacity(jewels_count);
    let mut sum = 0;

    for _ in 0..jewels_count {
        let weight: u64 = input.next().unwrap().trim().parse().unwrap();
        let value: u64 = input.next().unwrap().trim().parse().unwrap();
        jewels.push((weight, value));
    }
    // 무게기준 보석들 내림차순
    jewels.sort_by(|a, b| b.0.cmp(&a.0));

    for _ in 0..bag_count {
        let max: u64 = input.next().unwrap().trim().parse().unwrap();
        bag_heap.push(Reverse(max));
    }

    for _ in 0..bag_count {
        // 용량을 구함
        let max = bag_heap.pop().unwrap().0;

        // 보석상점에 무게가 가장 낮은 보석부터 차례대로 확인
        while let Some((w, v)) = jewels.pop() {
            // 만약 용량이 무게이상이라면
            if max >= w {
                // 최대힙에 가치를 넣음
                jewels_value_heap.push(v);
            } else {
                // 아니라면 이미 보석상점에 보석을 돌려놓고 검색을 멈춤
                jewels.push((w, v));
                break;
            }
        }

        // 가장 가치있는 보석을 sum에 더함
        if let Some(value) = jewels_value_heap.pop() {
            sum += value;
        }
    }

    // 값 출력
    stdout().write_all(format!("{}\n", sum).as_bytes()).unwrap();
}
