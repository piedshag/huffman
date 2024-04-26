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

fn compress(s: &str, huffman_codes: &HuffmanCodes) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut buffer = 0;
    let mut buffer_len = 0;

    for c in s.chars() {
        let code = huffman_codes.get(&(c as u8)).unwrap();
        for bit in code.chars() {
            buffer = (buffer << 1) | (bit.to_digit(10).unwrap() as u8);
            buffer_len += 1;

            if buffer_len == 8 {
                compressed.push(buffer);
                buffer = 0;
                buffer_len = 0;
            }
        }
    }

    if buffer_len > 0 {
        compressed.push(buffer << (8 - buffer_len));
    }

    compressed
}

fn decompress(compressed: &[u8], huffman_codes: &HuffmanCodes, output_len: usize) -> Vec<u8> {
    let mut decompressed = vec![];
    let mut buffer = String::new();

    'outer: for byte in compressed {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            buffer += bit.to_string().as_str();

            if let Some(symbol) = huffman_codes
                .iter()
                .find(|(_, code)| *code == buffer.as_str())
            {
                decompressed.push(*symbol.0);
                buffer.clear();

                if decompressed.len() == output_len {
                    break 'outer;
                }
            }
        }
    }

    decompressed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_huffman() {
        let test_string = "jjjjjjjjjjjjjjjjjjjjjjjjjjjjhuw8hwerh8wrhv8whe8vwdhjjjjjjjjjjjj";
        let huffman_codes = generate_huffman(test_string);
        let comressed = compress(test_string, &huffman_codes);

        println!("{:?}", huffman_codes);

        println!(
            "{:?}",
            test_string.as_bytes().len() as f32 / comressed.len() as f32
        );

        assert_eq!(
            test_string.as_bytes(),
            decompress(&comressed, &huffman_codes, test_string.len())
        );
    }
}
