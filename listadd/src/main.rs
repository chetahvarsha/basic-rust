use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::boxed::Box;
use std::option::Option;
use std::vec::Vec;

// Define the ListNode struct
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

// Implement methods for ListNode
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

// Implement PartialEq, Eq, PartialOrd, and Ord for ListNode to use it in a BinaryHeap
impl PartialEq for ListNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for ListNode {}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

// Function to merge multiple sorted linked lists
fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();

    // Initialize the heap with the head nodes of all the linked lists
    for list in lists {
        if let Some(node) = list {
            heap.push(node);
        }
    }

    // Dummy head to simplify the list merging
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;

    while let Some( node) = heap.pop() {
        // Add the smallest node to the merged list
        current.next = Some(node);
        current = current.next.as_mut().unwrap();

        // If the node has a next node, push it into the heap
        if let Some(next) = current.next.take() {
            heap.push(next);
        }
    }

    dummy.next
}

fn main() {
    // Helper function to create a linked list from a vector of values
    fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let  current = &mut head;

        for &val in values.iter().rev() {
            let node = Box::new(ListNode {
                val,
                next: current.take(),
            });
            *current = Some(node);
        }

        head
    }

    // Create example linked lists
    let lists = vec![
        create_list(vec![1, 4, 5]),
        create_list(vec![1, 3, 4]),
        create_list(vec![2, 6]),
    ];

    // Merge the linked lists
    let merged_list = merge_k_lists(lists);

    // Print the merged list
    let mut current = merged_list;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next;
    }
    println!();
}
