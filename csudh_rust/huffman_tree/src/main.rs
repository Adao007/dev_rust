use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Nodes for Huffman Tree
#[derive(Debug)]
struct Node {
    symbol: Option<char>,       // character in given alphabet
    frequency: usize,           // count of times in text
    left: Option<Box<Node>>,    // 0 code
    right: Option<Box<Node>>,   // 1 code 
}

// Counts the frequencies of a given input
fn count(input: &str) -> (Vec<char>, Vec<usize>) {
    let mut frequency_map = HashMap::new();

    for symbol in input.chars() {
        *frequency_map.entry(symbol).or_insert(0) += 1;  // Inserts symbol into hashmap OR increments count
    }

    let mut symbols = Vec::new();
    let mut frequencies = Vec::new();

    for (symbol, frequency) in frequency_map {
        symbols.push(symbol);                           // Inserts hashcode for the symbols 
        frequencies.push(frequency);                    // Inserts value in hashmap as the frequencies 
    }

    (symbols, frequencies)                              // Return symbols array and frequencies array 
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

fn grow_tree(symbols: &[char], frequencies: &[usize]) -> Option<Box<Node>> {
    #[derive(Debug)]
    struct HuffmanNode {                                // Nodes for our huffman tree
        node: Node, 
        tie: usize,                                     // Used for symbols with similar frequenices  
    }
    
    // OVERRIDING FUNCTIONS for comparisons 
    impl Ord for HuffmanNode {   
        fn cmp(&self, other: &Self) -> Ordering {
            other.node.frequency
                .cmp(&self.node.frequency)
                .then_with(|| self.tie.cmp(&other.tie)) // Use tie counter as tie-breaker
        }
    }

    impl PartialOrd for HuffmanNode {   
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for HuffmanNode {    
        fn eq(&self, other: &Self) -> bool {
            self.node.frequency == other.node.frequency && self.tie == other.tie
        }
    }

    impl Eq for HuffmanNode {}
    // OVERIDING FUNCTIONS END
    
    // Grow our Huffman Tree
    let mut min_heap: BinaryHeap<HuffmanNode> = BinaryHeap::new();

    for (i, &symbol) in symbols.iter().enumerate() {
        min_heap.push(HuffmanNode {
            node: Node {
                symbol: Some(symbol),
                frequency: frequencies[i],
                left: None,
                right: None,
            },
            tie: i,
        });
    }

    while min_heap.len() > 1 {
        let left = min_heap.pop().unwrap().node;
        let right = min_heap.pop().unwrap().node;

        let merged_node = Node {
            symbol: None,
            frequency: left.frequency + right.frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        min_heap.push(HuffmanNode { node: merged_node, tie: 0 }); 
    }

    min_heap.pop().map(|min_heap_node| Box::new(min_heap_node.node))
}

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

fn encode(text: &str, codes: &HashMap<char, String>) -> String {
    let mut encoded = String::new();
    for symbol in text.chars() {
        if let Some(code) = codes.get(&symbol) {
            encoded.push_str(code);
        }
    }
    encoded
}

fn decode(encoded_message: &str, huffman_tree: &Option<Box<Node>>) -> String {
    let mut decoded_message = String::new();
    let mut node = huffman_tree; 

    for secret in encoded_message.chars() {
        if let Some(ref n) = node {
            if secret == '0' {                  // If 0 move left 
                node = &n.left;
            } else {                            // Else 1 move right
                node = &n.right;
            }

            if let Some(ref n) = node {
                if let Some(symbol) = n.symbol {
                    decoded_message.push(symbol);
                    node = huffman_tree; 
                }
            }
        }
    }
    decoded_message                             // Return decoded string 
}

fn main() {
    
    /*      
     *      1.) First program that reads in a file, 
     *          counts the frequencies of symbols (returns arrays of frequencies and symbols), 
     *          creates a huffman tree (outputs the code), 
     *          then encodes the message.
     */

    // Input filename for huffman encoding
    print!("Filename: "); 
    io::stdout().flush().unwrap();  
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read line"); 
    println!();
    let file_name = file_name.trim();

    match read_file_content(file_name) {
        Ok(content) => {
            let (symbols, frequencies) = count(&content);
            let huffman_tree = grow_tree(&symbols, &frequencies);
            let mut codes = HashMap::new();
            
            // Outputs the huffman codes 
            code_generator(&huffman_tree, String::new(), &mut codes);
            println!("Huffman Codes: {:?}", codes); println!();
            
            // Encodes message using the huffman codes generated 
            let encoded_message = encode(&content, &codes);
            println!("Encoded Text: {}", encoded_message); println!();

    /*
     *      2.) Second program decodes the encoded message using the huffman codes.
     *          Outputs the message!
     */
            
            // Decodes message and outputs text
            let decoded_message = decode(&encoded_message, &huffman_tree); 
            println!("Decoded Text: {}", decoded_message); println!();
        }
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }
}
