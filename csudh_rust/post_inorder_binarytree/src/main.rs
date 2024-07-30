use std::rc::Rc;
use std::cell::RefCell;

/*  
 *  Node class with constructer(impl). Option variables (Left and Right Nodes)
 *  in rust takes either a value of "Some", or a value of "None". 
 *  (As a way to deal with null errors like from C++)  
 */
#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node{
   fn new(data: i32) -> Self {
        Node {
           data,
           left: None,
           right: None,
       }
   }
}

// Generate Tree for test cases
fn generate_tree(n: usize) -> Option<Rc<RefCell<Node>>> {
    if n == 0 {
        return None;
    }

    let mut nodes = Vec::new(); 
    for i in 0..n {
        nodes.push(Rc::new(RefCell::new(Node::new(i as i32))));
    }

    for i in 0..n {
        let node = nodes[i].clone();
        if 2 * i + 1 < n {
            node.borrow_mut().left = Some(nodes[2 * i + 1].clone());
        }
        if 2 * i + 2 < n {
            node.borrow_mut().right = Some(nodes[2 * i + 2].clone());
        }
    }

    Some(nodes[0].clone())
}

// output the inorder traversal
fn inorder_traversal(node: &Option<Rc<RefCell<Node>>>, traversal: &mut Vec<i32>) {
    if let Some(node_ref) = node {
        let node = node_ref.borrow();
        inorder_traversal(&node.left, traversal);
        traversal.push(node.data);
        inorder_traversal(&node.right, traversal);
    }
}

// output the postorder traversal
fn postorder_traversal(node: &Option<Rc<RefCell<Node>>>, traversal: &mut Vec<i32>) {
    if let Some(node_ref) = node {
        let node = node_ref.borrow();
        postorder_traversal(&node.left, traversal);
        postorder_traversal(&node.right, traversal);
        traversal.push(node.data);
    }
}


// Given that our largest n is 200 (guidelines), implemented a linear search function
fn search(arr: &[i32], start: usize, end: usize, data: i32) -> Option<usize> {
    for i in start ..=end {
        if arr[i] == data {
            return Some(i);
        }
    }
    None
}

fn build_binary_tree( inorder: &[i32], postorder: &[i32], i_start: usize, i_end: usize, 
                      p_start: usize, p_end: usize) -> Option<Rc<RefCell<Node>>> {
    // Base Case
    if i_start > i_end {
        return None;
    }

    // chosen node from postorder new p_end 
    let node_data = postorder[p_end];
    let node = Rc::new(RefCell::new(Node::new(node_data)));

    // Returns if the node is leaf node
    if i_start == i_end {
        return Some(node); 
    }

    let i_index = search(inorder, i_start, i_end, node_data)?; 

    if i_index == 0 {
        node.borrow_mut().left = None;
    }
    else {
        // inorder index used for creating left and right subtrees 
        node.borrow_mut().left = build_binary_tree( 
            inorder, 
            postorder, 
            i_start,             
            i_index - 1,
            p_start, 
            p_start + i_index - i_start - 1, 
        );
   }
        
    node.borrow_mut().right = build_binary_tree(
        inorder, 
        postorder, 
        i_index + 1, 
        i_end, 
        p_start + i_index - i_start, 
        p_end - 1,
    ); 
    
    Some(node) // Return Recursive Calls 
}

fn main() {
    // Create timer variable
    use std::time::Instant;
    let now = Instant::now();

    let mut timer_before = now.elapsed();
    // Test Case 1: Empty
    test_case(vec![], vec![], "Empty Tree");  
    let mut timer_after = now.elapsed(); 
    println!("Time elapsed: {:.2?}", (timer_after - timer_before));
    println!(); 

    // Test Case 2: Invalid Tree
    timer_before = now.elapsed();
    test_case(vec![1, 2, 3], vec![1, 2, 3, 4], "Invalid Tree");
    timer_after = now.elapsed(); 
    println!("Time elapsed: {:.2?}", (timer_after - timer_before)); 
    println!();

    // Test Case 3: n = 20
    timer_before = now.elapsed();
    setup_larger(20);  
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - timer_before));
    println!();

    // Test Case 4: n = 50
    timer_before = now.elapsed();
    setup_larger(50);
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - timer_before));
    println!();

    // Test Case 5: n = 100
    timer_before = now.elapsed();
    setup_larger(100);
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - timer_before));
    println!();

    // Test Case 6: n = 200
    timer_before = now.elapsed();
    setup_larger(200);
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - timer_before));
    println!();
}

// Prints out the constructed Binary Tree in Preorder
fn construct_results(node: &Option<Rc<RefCell<Node>>>) {
    if let Some (node_ref) = node {
        let node = node_ref.borrow(); 
        print!("{} ", node.data);
        construct_results(&node.left);
        construct_results(&node.right);
    }
}

/* TEST SUITE FUNCTIONS */ 
fn test_case(inorder: Vec<i32>, postorder: Vec<i32>, case: &str) {
    println!("Test Case: {}", case);  
    println!("Inorder Traversal: {:?}", inorder); 
    println!("Postorder Traversal: {:?}", postorder); 
    
    // Returns if tree is an empty tree
    if inorder.len() == 0 {
        println!("Empty Tree: length of list are 0!");
        return 
    } 
    else if inorder.len() != postorder.len() {
        println!("No solution!"); 
        return 
    }

    let n = inorder.len();
    let root = build_binary_tree(&inorder, &postorder, 0, n - 1, 0, n - 1);
    print!("Preorder of Constructed Binary Tree: "); 
    construct_results(&root); 
    println!("\n"); 
}

fn setup_larger(n: usize) {
    let case_type = n.to_string().as_str().to_owned() + " Case"; 
    let tree = generate_tree(n); 
    let mut inorder = Vec::new();
    let mut postorder = Vec::new(); 

    inorder_traversal(&tree, &mut inorder);
    postorder_traversal(&tree, &mut postorder); 
    test_case(inorder, postorder, &case_type); 
}
