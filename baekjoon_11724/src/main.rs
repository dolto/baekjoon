use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
};

struct Node {
    nodes: Vec<usize>,
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<usize>());

    let nodes_count = input.next().unwrap();
    let mut nodes = Vec::with_capacity(nodes_count);
    for _ in 0..nodes_count {
        nodes.push(Node {
            nodes: Vec::with_capacity(nodes_count),
        });
    }

    let connect_count = input.next().unwrap();
    for _ in 0..connect_count {
        let left = input.next().unwrap() - 1;
        let right = input.next().unwrap() - 1;
        nodes[left].nodes.push(right);
        nodes[right].nodes.push(left);
    }

    let mut connected_components = 0;
    let mut check_set = HashSet::with_capacity(nodes_count);
    while check_set.len() < nodes_count {
        connected_components += 1;
        let mut node = 0;
        for i in 0..nodes_count {
            if !check_set.contains(&i) {
                node = i;
                break;
            }
        }
        find_component_group(&mut check_set, &nodes, node);
        // println!("{:?}", check_set);
    }

    stdout()
        .write_all(format!("{connected_components}").as_bytes())
        .unwrap();
}

fn find_component_group(temp_set: &mut HashSet<usize>, nodes: &Vec<Node>, node: usize) {
    if temp_set.contains(&node) {
        return;
    }
    temp_set.insert(node);
    for i in 0..nodes[node].nodes.len() {
        find_component_group(temp_set, nodes, nodes[node].nodes[i]);
    }
}
