use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

// A Tree node structure
#[derive(Debug, Clone)]
struct Node {
    ch: Option<char>,
    freq: usize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(ch: Option<char>, freq: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            ch,
            freq,
            left: None,
            right: None,
        }))
    }
}

// Implement Ord, PartialOrd, PartialEq, Eq for Node to use it in BinaryHeap
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

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for Node {}

// Function to check if the node is a leaf node
fn is_leaf(node: &Rc<RefCell<Node>>) -> bool {
    node.borrow().left.is_none() && node.borrow().right.is_none()
}

// Traverse the Huffman Tree and store Huffman Codes in a map
fn encode(node: Rc<RefCell<Node>>, prefix: String, huffman_code: &mut HashMap<char, String>) {
    let node_ref = node.borrow();
    if let Some(ch) = node_ref.ch {
        huffman_code.insert(ch, if prefix.is_empty() { "1".to_string() } else { prefix.clone() });
    }

    if let Some(ref left) = node_ref.left {
        encode(left.clone(), prefix.clone() + "0", huffman_code);
    }

    if let Some(ref right) = node_ref.right {
        encode(right.clone(), prefix + "1", huffman_code);
    }
}

// Traverse the Huffman Tree and decode the encoded string
fn decode(node: Rc<RefCell<Node>>, mut index: usize, encoded_text: &str) -> (Option<char>, usize) {
    let node_ref = node.borrow();
    if is_leaf(&node) {
        return (node_ref.ch, index);
    }

    index += 1;

    if index < encoded_text.len() {
        if &encoded_text[index..=index] == "0" {
            if let Some(ref left) = node_ref.left {
                return decode(left.clone(), index, encoded_text);
            }
        } else {
            if let Some(ref right) = node_ref.right {
                return decode(right.clone(), index, encoded_text);
            }
        }
    }

    (None, index)
}

// Builds the Huffman Tree and decodes the given input text
fn build_huffman_tree(text: &str) {
    if text.is_empty() {
        return;
    }

    // Count the frequency of appearance of each character
    let mut freq_map = HashMap::new();
    for ch in text.chars() {
        *freq_map.entry(ch).or_insert(0) += 1;
    }

    // Create a priority queue to store live nodes of the Huffman tree
    let mut pq = BinaryHeap::new();

    // Create a leaf node for each character and add it to the priority queue
    for (ch, freq) in &freq_map {
        pq.push(Node::new(Some(*ch), *freq));
    }

    // Do till there is more than one node in the queue
    while pq.len() > 1 {
        // Remove the two nodes of the highest priority (the lowest frequency) from the queue
        let left = pq.pop().unwrap();
        let right = pq.pop().unwrap();

        // Create a new internal node with these two nodes as children
        // and with a frequency equal to the sum of both nodes' frequencies
        let sum_freq = left.borrow().freq + right.borrow().freq;
        let new_node = Node::new(None, sum_freq);
        new_node.borrow_mut().left = Some(left);
        new_node.borrow_mut().right = Some(right);

        // Add the new node to the priority queue
        pq.push(new_node);
    }

    // `root` stores pointer to the root of the Huffman Tree
    let root = pq.pop().unwrap();

    // Traverse the Huffman tree and store the Huffman codes in a map
    let mut huffman_code = HashMap::new();
    encode(root.clone(), "".to_string(), &mut huffman_code);

    // Print the Huffman codes
    println!("Huffman Codes are: {:?}", huffman_code);
    println!("The original string is: {}", text);

    // Encode the input text
    let mut encoded_text = String::new();
    for ch in text.chars() {
        encoded_text += &huffman_code[&ch];
    }
    println!("The encoded string is: {}", encoded_text);

    // Decode the encoded text
    println!("The decoded string is: ");
    let mut index = 0;
    while index < encoded_text.len() {
        let (decoded_char, new_index) = decode(root.clone(), index, &encoded_text);
        if let Some(ch) = decoded_char {
            print!("{}", ch);
        }
        index = new_index;
    }
    println!();
}

fn main() {
    let text = "Huffman coding is a data compression algorithm.";
    build_huffman_tree(text);
}

