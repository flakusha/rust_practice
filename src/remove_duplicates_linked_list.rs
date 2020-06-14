/// Given a sorted linked list, delete all duplicates such that each element
/// appear only **once**
/// Input: 1->1->2->3->3
/// Output: 1->2->3

use crate::vec_to_ll;
use crate::ll_to_vec;
use crate::ListNode;
use crate::LinkedList;

pub fn rem_dupl_ll(input: Vec<i32>) {

struct Solution;

impl Solution {
    /// Removes duplicates from linked list
    pub fn delete_duplicates(head: LinkedList) -> LinkedList {
        if head.is_none() {return None;}

        let mut ll = &head;
        let mut result: LinkedList = None;
        let mut current = &mut result;
        let mut added_first = false;

        while ll.is_some() {
            if let Some(node) = ll {
                let val_cur = node.val;
                if added_first == false {
                    *current = Some(Box::new(ListNode::new(val_cur)));
                    current = &mut current.as_mut().unwrap().next;
                    added_first = true;
                }
                if ll.as_ref().unwrap().next.is_some() {
                    let val_next = ll.as_ref().unwrap().next
                    .as_ref().unwrap().val;
                    if val_cur != val_next {
                        *current = Some(Box::new(ListNode::new(val_next)));
                        current = &mut current.as_mut().unwrap().next;
                    }
                }
                ll = &node.next;
            }
        }
        result
    }

    pub fn delete_duplicates_vec(head: LinkedList) -> LinkedList {
        if head.is_none() {return None;}
        let mut vec = ll_to_vec(head);
        vec.dedup();
        println!("{:?}", vec);
        let ll = vec_to_ll(vec);
        ll
    }
}
let input_01 = input.clone();
let output_00 = Solution::delete_duplicates(vec_to_ll(input));
let output_01 = Solution::delete_duplicates_vec(vec_to_ll(input_01));
println!("{:?}", output_00);
println!("{:?}", output_01);
assert_eq!(output_00, output_01);
}