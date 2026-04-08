use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node<T: Ord> {
    data: T,
    children: Vec<Node<T>>
}

impl<T: Ord + Copy + Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, data: T) {
        self.children.push(Node::new(data))
    }
}

/// https://www.geeksforgeeks.org/dsa/introduction-to-dynamic-programming-on-trees/
pub fn run() {
    let mut root = Node::new(3);

    root.add_child(2);
    root.add_child(1);
    root.add_child(10);

    // left
    root.children[0].add_child(1);
    root.children[0].add_child(3);
    root.children[0].children[0].add_child(4);
    root.children[0].children[0].add_child(5);

    // middle
    root.children[1].add_child(9);
    root.children[1].children[0].add_child(9);
    root.children[1].children[0].add_child(8);

    // right
    root.children[2].add_child(1);
    root.children[2].add_child(5);
    root.children[2].add_child(3);
    let mut sums: Vec<u64> = Vec::new();
    max_sum_path_top_down(&root, 0, &mut sums);
    println!("max {sums:?}");

    let mut memo: HashMap<Node<u64>, u64> = HashMap::new();
    let max_node_path = max_sum_path_bottom_up_dp(&root, &mut memo);
    println!("max {max_node_path}");
    for (k, v) in memo {
        println!("v {v}");
    }
}

fn max_sum_path_top_down(node: &Node<u64>, mut current_sum: u64, sums: &mut Vec<u64>) {
    current_sum += node.data;

    if node.children.is_empty() {
        sums.push(current_sum);
        return;
    }

    for child in &node.children {
        max_sum_path_top_down(child, current_sum, sums);
    }
}

fn max_sum_path_bottom_up_dp(node: &Node<u64>, memo: &mut HashMap<Node<u64>, u64>) -> u64 {
    if let Some(&val) = memo.get(node) {
        return val;
    }

    if node.children.is_empty() {
        memo.insert(node.clone(), node.data);
        return node.data
    }

    let mut max_sums = 0;
    for child in &node.children {
        let sum = max_sum_path_bottom_up_dp(child, memo);
        max_sums = max_sums.max(sum);
    }

    memo.insert(node.clone(), max_sums + node.data);
    max_sums + node.data
}