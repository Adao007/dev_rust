use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

// A Tree node structure
#[derive(Debug, Clone)]
struct Node {
    symbol: Option<char>,
    frequency: usize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(symbol: Option<char>, frequency: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            symbol,
            frequency,
            left: None,
            right: None,
        }))
    }
}

// Implement Ord, PartialOrd, PartialEq, Eq for Node to use it in BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
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
    if let Some(ch) = node_ref.symbol {
        huffman_code.insert(ch, if prefix.is_empty() { "1".to_string() } else { prefix.clone() });
    }

    if let Some(ref left) = node_ref.left {
        encode(left.clone(), prefix.clone() + "0", huffman_code);
    }

    if let Some(ref right) = node_ref.right {
        encode(right.clone(), prefix + "1", huffman_code);
    }
}

fn decode(mut node: Rc<RefCell<Node>>, mut index: usize, encoded_text: &str) -> (Option<char>, usize) {
    while !is_leaf(&node) {
        if index >= encoded_text.len() {
            return (None, index);
        }

        let current_node = node.clone();
        node = if &encoded_text[index..=index] == "0" {
            current_node.borrow().left.as_ref().unwrap().clone()
        } else {
            current_node.borrow().right.as_ref().unwrap().clone()
        };

        index += 1;
    }

    (node.borrow().symbol, index)
}

// Builds the Huffman Tree and decodes the given input text
fn grow_tree(text: &str) {
    if text.is_empty() {
        return;
    }

    // Count the frequency of appearance of each character
    let mut frequency_map = HashMap::new();
    for ch in text.chars() {
        *frequency_map.entry(ch).or_insert(0) += 1;
    }

    // Create a priority queue to store live nodes of the Huffman tree
    let mut pq = BinaryHeap::new();

    // Create a leaf node for each character and add it to the priority queue
    for (symbol, frequency) in &frequency_map {
        pq.push(Node::new(Some(*symbol), *frequency));
    }

    // Do till there is more than one node in the queue
    while pq.len() > 1 {
        // Remove the two nodes of the highest priority (the lowest frequency) from the queue
        let left = pq.pop().unwrap();
        let right = pq.pop().unwrap();

        // Create a new internal node with these two nodes as children
        // and with a frequency equal to the sum of both nodes' frequencies
        let sum_frequency = left.borrow().frequency + right.borrow().frequency;
        let new_node = Node::new(None, sum_frequency);
        new_node.borrow_mut().left = Some(left);
        new_node.borrow_mut().right = Some(right);

        // Add the new node to the priority queue
        pq.push(new_node);
    }

    let root = pq.pop().unwrap();

    let mut code = HashMap::new();
    encode(root.clone(), "".to_string(), &mut code);

    println!("Huffman Codes are: {:?}", code);
    println!("The original string is: {}", text);

    // Encode the input text
    let mut encoded_text = String::new();
    for symbols in text.chars() {
        encoded_text += &code[&symbols];
    }
    println!("The encoded string is: {}", encoded_text);

    print!("The decoded string is: ");
    let mut index = 0;
    while index < encoded_text.len() {
        let (decoded_char, new_index) = decode(root.clone(), index, &encoded_text);
        if let Some(symbol) = decoded_char {
            print!("{}", symbol);
        }
        index = new_index;
    }
    println!();
}

fn main() {
    let english_text = "A quick brown fox jumped over the lazy dog";
    grow_tree(english_text); // Function will create the huffman tree, encode the message, and then decode it! 
}

