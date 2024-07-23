use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

// 힙 모레시계 방법을 이용
// min힙과 max힙을 둘 다 만들고, min힙의 top이 언제나 max힙보다 커야하고
// max힙의 요소 개수는 min힙과 1보다 크거나 같아야한다는 조건만 갖추면, max힙의 top값은 항상 가운데 값이 됨
fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut reader = BufReader::new(stdin().lock());

    reader.read_line(&mut input).unwrap();
    let count = input.trim().parse().unwrap();
    let mut min_heap = BinaryHeap::with_capacity(count);
    let mut max_heap = BinaryHeap::with_capacity(count);

    for _ in 0..count {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let res: i32 = input.trim().parse().unwrap();
        if let (Some(_), Some(Reverse(_))) = (max_heap.peek(), min_heap.peek()) {
            let min_len = min_heap.len();
            let max_len = max_heap.len();
            if min_len == max_len {
                max_heap.push(res);
            } else {
                min_heap.push(Reverse(res));
            }
            if *max_heap.peek().unwrap() > min_heap.peek().unwrap().0 {
                let max = max_heap.pop().unwrap();
                let min = min_heap.pop().unwrap();
                min_heap.push(Reverse(max));
                max_heap.push(min.0);
            }
        } else if let Some(_) = max_heap.peek() {
            min_heap.push(Reverse(res));
            if *max_heap.peek().unwrap() > min_heap.peek().unwrap().0 {
                let max = max_heap.pop().unwrap();
                let min = min_heap.pop().unwrap();
                min_heap.push(Reverse(max));
                max_heap.push(min.0);
            }
        } else {
            max_heap.push(res);
        }
        output += format!("{}\n", max_heap.peek().unwrap()).as_str();
    }
    stdout().write_all(output.as_bytes()).unwrap();
}
