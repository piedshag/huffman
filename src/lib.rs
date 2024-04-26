use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    fmt::Formatter,
};

type NodeRef = Option<Box<Node>>;
type HuffmanCodes = HashMap<u8, HuffmanCode>;

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

#[derive(Default, PartialEq, Clone)]
struct HuffmanCode {
    code: u8,
    len: u8,
}

impl Debug for HuffmanCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0width$b}", self.code, width = self.len as usize)
    }
}

impl HuffmanCode {
    fn push(mut self, bit: u8) -> Self {
        self.code = (self.code << 1) | bit;
        self.len += 1;
        self
    }

    fn bits(&self) -> Vec<u8> {
        (0..self.len)
            .map(|i| (self.code >> (self.len - i - 1)) & 1)
            .collect()
    }

    fn clear(&mut self) {
        self.code = 0;
        self.len = 0;
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
    create_codes(HuffmanCode::default(), root.0, &mut huffman_codes);

    debug_assert_eq!(heap_len, huffman_codes.len());

    huffman_codes
}

fn create_codes(code: HuffmanCode, mut root: Node, codes: &mut HuffmanCodes) {
    while let (Some(left), Some(right)) = (root.left.take(), root.right.take()) {
        create_codes(code.clone().push(0), *left, codes);
        create_codes(code.clone().push(1), *right, codes);
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
        for bit in code.bits() {
            buffer = (buffer << 1) | bit;
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
    let mut buffer = HuffmanCode::default();

    'outer: for byte in compressed {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            buffer = buffer.push(bit);

            if let Some(symbol) = huffman_codes.iter().find(|(_, code)| *code == &buffer) {
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
        let test_string = "huihuihyf7d6d6d64s4seseseawa23q2a4";
        let huffman_codes = generate_huffman(test_string);
        let comressed = compress(test_string, &huffman_codes);

        huffman_codes.iter().for_each(|(k, v)| {
            println!("{:?} {:?}", *k as char, v);
        });

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
