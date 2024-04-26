use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

type NodeRef = Option<Box<Node>>;
type HuffmanCodes = HashMap<u8, String>;

#[derive(PartialEq, Debug, Eq, Clone)]
struct Node {
    left: NodeRef,
    right: NodeRef,
    weight: i64,
    symbol: Option<u8>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn new(weight: i64, symbol: u8) -> Self {
        Node {
            left: None,
            right: None,
            weight,
            symbol: Some(symbol),
        }
    }
}

fn generate_huffman(s: &str) -> HuffmanCodes {
    let mut min_heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    s.chars()
        .fold(HashMap::new(), |mut acc, c| {
            let count = acc.get(&c).unwrap_or(&0) + 1;
            acc.insert(c, count);
            acc
        })
        .into_iter()
        .for_each(|(c, count)| min_heap.push(Reverse(Node::new(count as i64, c as u8))));

    let heap_len = min_heap.len();

    let root = loop {
        match (min_heap.pop(), min_heap.pop()) {
            (Some(node), Some(node1)) => {
                let new_node = Node {
                    left: Some(Box::new(node.0.clone())),
                    right: Some(Box::new(node1.0.clone())),
                    weight: node.0.weight + node1.0.weight,
                    symbol: None,
                };

                min_heap.push(Reverse(new_node));
            }
            (Some(root), _) => break root,
            _ => panic!("no root found"),
        }
    };

    let mut huffman_codes = HashMap::new();
    create_codes(String::new(), root.0, &mut huffman_codes);

    debug_assert_eq!(heap_len, huffman_codes.len());

    huffman_codes
}

fn create_codes(code: String, mut root: Node, codes: &mut HuffmanCodes) {
    while let (Some(left), Some(right)) = (root.left.take(), root.right.take()) {
        create_codes(code.clone() + "0", *left, codes);
        create_codes(code.clone() + "1", *right, codes);
    }

    if let Some(symbol) = root.symbol {
        codes.insert(symbol, code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_huffman() {
        let test_string = "this is a test string";
        let huffman_codes = generate_huffman(test_string);

        huffman_codes.iter().for_each(|(k, v)| {
            println!("{}: {}", char::from(*k), v);
        });
    }
}
