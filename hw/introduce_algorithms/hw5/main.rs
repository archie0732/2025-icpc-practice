use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    ch: Option<char>,
    freq: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn new_leaf(ch: char, freq: i32) -> Self {
        Node {
            ch: Some(ch),
            freq,
            left: None,
            right: None,
        }
    }

    fn new_internal(left: Node, right: Node) -> Self {
        Node {
            ch: None,
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

fn generate_codes(node: &Node, prefix: String, codes: &mut Vec<(char, String)>) {
    if let Some(c) = node.ch {
        codes.push((c, prefix));
    } else {
        if let Some(ref l) = node.left {
            generate_codes(l, prefix.clone() + "0", codes);
        }
        if let Some(ref r) = node.right {
            generate_codes(r, prefix.clone() + "1", codes);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n_str = tokens.next().unwrap();
    let n: usize = n_str.parse().unwrap();

    let mut heap = BinaryHeap::new();

    for _ in 0..n {
        let char_str = tokens.next().unwrap();
        let freq_str = tokens.next().unwrap();

        let ch = char_str.chars().next().unwrap();
        let freq: i32 = freq_str.parse().unwrap();

        heap.push(Node::new_leaf(ch, freq));
    }

    while heap.len() > 1 {
        let left_child = heap.pop().unwrap();
        let right_child = heap.pop().unwrap();

        let parent = Node::new_internal(left_child, right_child);
        heap.push(parent);
    }

    let root = heap.pop();

    let mut codes: Vec<(char, String)> = Vec::new();
    if let Some(root_node) = root {
        if root_node.left.is_none() && root_node.right.is_none() {
            if let Some(c) = root_node.ch {
                codes.push((c, "0".to_string()));
            }
        } else {
            generate_codes(&root_node, String::new(), &mut codes);
        }
    }

    codes.sort_by(|a, b| a.0.cmp(&b.0));

    for (ch, code) in codes {
        println!("{}: {}", ch, code);
    }
}
