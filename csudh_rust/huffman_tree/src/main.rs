use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Define the Huffman Node structure
#[derive(Debug)]
struct Node {
    symbol: Option<char>,
    frequency: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Counts the frequencies of a given input
fn count(input: &str) -> (Vec<char>, Vec<usize>) {
    let mut frequency_map = HashMap::new();

    for ch in input.chars() {
        *frequency_map.entry(ch).or_insert(0) += 1;
    }

    let mut symbols = Vec::new();
    let mut frequencies = Vec::new();

    for (ch, count) in frequency_map {
        symbols.push(ch);
        frequencies.push(count);
    }

    (symbols, frequencies)
}

// Function to read in file
fn read_file_content(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut content = String::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        content.push_str(&line?);
    }

    Ok(content)     
}

fn build_huffman_tree(symbols: &[char], frequencies: &[usize]) -> Option<Box<Node>> {
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;

    #[derive(Debug)]
    struct HeapNode {
        node: Node,
        index: usize,  // Add an index to ensure stability
    }

    impl Ord for HeapNode {
        fn cmp(&self, other: &Self) -> Ordering {
            other.node.frequency
                .cmp(&self.node.frequency)
                .then_with(|| self.index.cmp(&other.index)) // Use index as tie-breaker
        }
    }

    impl PartialOrd for HeapNode {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for HeapNode {
        fn eq(&self, other: &Self) -> bool {
            self.node.frequency == other.node.frequency && self.index == other.index
        }
    }

    impl Eq for HeapNode {}

    let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();

    for (i, &ch) in symbols.iter().enumerate() {
        heap.push(HeapNode {
            node: Node {
                symbol: Some(ch),
                frequency: frequencies[i],
                left: None,
                right: None,
            },
            index: i, // Use the index in the original data as the tie-breaker
        });
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap().node;
        let right = heap.pop().unwrap().node;

        let merged_node = Node {
            symbol: None,
            frequency: left.frequency + right.frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        heap.push(HeapNode { node: merged_node, index: 0 }); // Index 0 for internal nodes
    }

    heap.pop().map(|heap_node| Box::new(heap_node.node))
}

// Generate Huffman Codes
fn code_generator(node: &Option<Box<Node>>, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(n) = node {
        if let Some(symbol) = n.symbol {
            codes.insert(symbol, prefix);
        } else {
            code_generator(&n.left, format!("{}0", prefix), codes);
            code_generator(&n.right, format!("{}1", prefix), codes);
        }
    }
}

// Decode Huffman encoded string
fn decode(encoded: &str, root: &Option<Box<Node>>) -> String {
    let mut result = String::new();
    let mut current = root;

    for bit in encoded.chars() {
        if let Some(ref node) = current {
            current = if bit == '0' {
                &node.left
            } else {
                &node.right
            };

            if let Some(ref leaf_node) = current {
                if let Some(symbol) = leaf_node.symbol {
                    result.push(symbol);
                    current = root;
                }
            }
        }
    }

    result
}

fn main() {
    // Input filename for huffman encoding
    print!("Filename: ");
    io::stdout().flush().unwrap();
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read line");
    let file_name = file_name.trim();

    match read_file_content(file_name) {
        Ok(content) => {
            let (symbols, frequencies) = count(&content);

            let huffman_tree = build_huffman_tree(&symbols, &frequencies);

            let mut codes = HashMap::new();
            code_generator(&huffman_tree, String::new(), &mut codes);

            println!("Huffman Codes: {:?}", codes);

            let encoded_input = "0001110"; // Example encoded input
            let decoded_output = decode(encoded_input, &huffman_tree);

            println!("Decoded output: {}", decoded_output);
        }
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }
}

