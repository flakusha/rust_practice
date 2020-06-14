use crate::vec_to_ll;

pub fn rem_dupl_ll(input: Vec<i32>) {

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

type LinkedList = Option<Box<ListNode>>;
struct Solution;

impl Solution {
  pub fn delete_duplicates(head: LinkedList) -> LinkedList {
      
  }
}
let mut result = Solution::delete_duplicates(vec_to_ll(input));
println!("{:?}", result);
}