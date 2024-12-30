struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}


fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
    // Step 1: Calculate the length of the list
    let mut length = 0;
    let mut current = &head;

    while let Some(node) = current {
        length += 1;
        current = &node.next;
    }

    // Determine the position to remove from the start
    let mut remove_pos = length - n;

    // Handle edge case where we remove the head of the list
    if remove_pos == 0 {
        return head.and_then(|node| node.next);
    }

    // Step 2: Traverse to the node just before the one to remove
    let mut current = &mut head;
    
    while remove_pos > 1 {
        if let Some(ref mut node) = current {
            current = &mut node.next;
        }
        remove_pos -= 1;
    }

    // Step 3: Remove the node
    if let Some(ref mut node) = current {
        node.next = node.next.take().and_then(|next_node| next_node.next);
    }

    head
}

fn main() {
    // Create a linked list: 1 -> 2 -> 3 -> 4 -> 5
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None,
                    })),
                })),
            })),
        })),
    }));

    // Remove the 2nd node from the end (should remove 4)
    let new_head = remove_nth_from_end(head, 2);

    // Print the new list
    let mut current = new_head;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next;
    }
    println!("None");
}
