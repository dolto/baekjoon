// 아이디어
// Rc<i32>로 배열을 저장 (내림차순)
// 저장된 데이터들은 인덱스로 볼 수 있음
// 저장된 데이터에서 먼저풀면 좋은 문제의 쌍을 HashMap<Rc<i32>, Rc<i32>> 에 저장
// 데이터를 순차적으로 순회하면서 레퍼값이 2이상이라면 HashMap을 참조해서 value먼저 풀게시키고 HashMap에 없으면 출력에서 제외시키면 됨

// 문제점
// 먼저풀면 좋은문제가 여러개인 경우가 있음
// 또한 먼저풀면 좋은 문제를 먼저 풀면 좋은문제가 있는 등, 연결되어있는 경우 감지를 못함
// 순차적으로 문제를 풀려다가 먼저 풀어야 할 문제가 발견되면, 일단 쟁여놓고, 다음문제를 풀고나서, 먼저 풀 문제를 풀면 그 다음에 쟁여놓은 문제를 푸는 형태가 맞음
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io::{stdin, stdout, BufReader, Read, Write},
    rc::Rc,
};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let pcount = input.next().unwrap().trim().parse().unwrap();
    let scount = input.next().unwrap().trim().parse().unwrap();

    let mut probloms = Vec::with_capacity(pcount);
    let mut needs_key: HashMap<Rc<i32>, Vec<Rc<i32>>> = HashMap::with_capacity(scount);
    let mut solve_set = HashSet::with_capacity(scount);

    for i in 0..pcount as i32 {
        probloms.push(Rc::new(i + 1));
    }
    for _ in 0..scount {
        let a: usize = input.next().unwrap().parse().unwrap();
        let b: usize = input.next().unwrap().parse().unwrap();

        let a = Rc::clone(&probloms[a - 1]);
        let b = Rc::clone(&probloms[b - 1]);
        // if a > b {
        if let Some(i) = needs_key.get_mut(&b) {
            i.push(a);
        } else {
            let mut temp = Vec::with_capacity(scount);
            temp.push(a);
            needs_key.insert(b, temp);
        }
        // }
    }

    let mut probloms_heap = BinaryHeap::with_capacity(pcount);
    for p in probloms {
        probloms_heap.push(Reverse(p));
    }

    let mut temp_probloms = Vec::with_capacity(pcount);
    while !probloms_heap.is_empty() || !temp_probloms.is_empty() {
        if let Some(Reverse(p)) = probloms_heap.pop() {
            if let Some(key) = needs_key.get_mut(&p) {
                let mut is_empty = true;
                for i in key.iter() {
                    if !solve_set.contains(i) {
                        is_empty = false;
                    }
                }
                if is_empty {
                    needs_key.remove(&p);
                    if !solve_set.contains(&p) {
                        solve_set.insert(Rc::clone(&p));
                        output.push_str(format!("{} ", p).as_str());
                    }
                } else {
                    // println!("push {}", p);
                    temp_probloms.push(Reverse(p));
                }
            } else {
                while let Some(temp) = temp_probloms.pop() {
                    // println!("pop {}", temp.0);
                    probloms_heap.push(temp);
                }
                if !solve_set.contains(&p) {
                    solve_set.insert(Rc::clone(&p));
                    output.push_str(format!("{} ", p).as_str());
                }
            }
        } else {
            while let Some(temp) = temp_probloms.pop() {
                // println!("pop {}", temp.0);
                probloms_heap.push(temp);
            }
        }

        // while let Some(Reverse(p)) = temp_probloms.pop() {
        //     if let Some(key) = needs_key.get_mut(&p) {
        //         let mut is_empty = true;
        //         for i in key.iter() {
        //             if !solve_set.contains(i) {
        //                 is_empty = false;
        //             }
        //         }
        //         if is_empty {
        //             needs_key.remove(&p);
        //             solve_set.insert(p);
        //             output.push_str(format!("{} ", p).as_str());
        //         } else {
        //             probloms.push(Reverse(p));
        //         }
        //     } else {
        //         if !solve_set.contains(&p) {
        //             solve_set.insert(p);
        //             output.push_str(format!("{} ", p).as_str());
        //         }
        //     }
        // }
    }

    stdout().write_all(output.trim().as_bytes()).unwrap();
}
