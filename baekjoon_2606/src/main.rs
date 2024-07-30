use std::{
    cell::RefCell,
    collections::HashSet,
    io::{stdin, stdout, BufReader, BufWriter, Read, Write},
    rc::Rc,
};

struct Node {
    number: usize,
    connect: Vec<Rc<RefCell<Node>>>,
}
fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut writer = BufWriter::new(stdout().lock());

    let count: usize = input.next().unwrap().trim().parse().unwrap();
    let mut nodes = Vec::with_capacity(count);
    for i in 0..count {
        nodes.push(Rc::new(RefCell::new(Node {
            connect: vec![],
            number: i,
        })));
    }

    let connects = input.next().unwrap().trim().parse().unwrap();
    for _ in 0..connects {
        let left: usize = input.next().unwrap().trim().parse().unwrap();
        let right: usize = input.next().unwrap().trim().parse().unwrap();

        nodes[left - 1]
            .borrow_mut()
            .connect
            .push(Rc::clone(&nodes[right - 1]));
        nodes[right - 1]
            .borrow_mut()
            .connect
            .push(Rc::clone(&nodes[left - 1]));
    }

    let mut virous = HashSet::with_capacity(count);
    search(0, &nodes, &mut virous);

    virous.remove(&0);

    writer
        .write_all(format!("{}", virous.len()).as_bytes())
        .unwrap();
}

fn search(index: usize, nodes: &Vec<Rc<RefCell<Node>>>, virous: &mut HashSet<usize>) {
    if !virous.contains(&index) {
        virous.insert(index);
        nodes[index].borrow().connect.iter().for_each(|node| {
            let index = node.borrow().number;
            search(index, nodes, virous);
        });
    }
}
