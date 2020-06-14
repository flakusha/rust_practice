/// Algorithm sums two linked lists of integers in reversed order

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

// Shortcut
pub type LinkedList = Option<Box<ListNode>>;

/// Converts vector to linked list
pub fn vec_to_ll(input_vector: Vec<i32>) -> LinkedList {
    // Empty linked list in the beginning
    let mut ll = None;
    // Iterate in reverse through vector
    for &vector in input_vector.iter().rev() {
        // Add nodes one by one
        let mut node = ListNode::new(vector);
        node.next = ll;
        ll = Some(Box::new(node));
    }
    // Return the result
    ll
}

pub fn ll_to_vec(input_ll: LinkedList) -> Vec<i32> {
    let mut ll = &input_ll;
    let mut vec = <Vec<i32>>::new();
    while ll.is_some() {
        if let Some(node) = ll {
            vec.push(node.val);
            ll = &node.next;
        }
    }
    println!("{:?}", vec);
    vec
}

pub fn add_two_numbers_linked_list(input_01: Vec<i32>, input_02: Vec<i32>) {

#[derive(Debug)]
struct Solution {l1: LinkedList, l2: LinkedList}

impl Solution {
    pub fn add_two_numbers(l1: LinkedList, l2: LinkedList) -> LinkedList {
        let mut list_01 = &l1;
        let mut list_02 = &l2;
        let mut addition = 0;
        let mut result = None;
        let mut current = &mut result;

        while list_01.is_some() || list_02.is_some() || addition != 0 {
          let mut result_temp = addition;
          
          if let Some(node) = list_01 {
            result_temp += node.val;
            list_01 = &node.next;
          }
          if let Some(node) = list_02 {
            result_temp += node.val;
            list_02 = &node.next;
          }
          addition = result_temp / 10;
          *current = Some(Box::new(ListNode::new(result_temp % 10)));
          current = &mut current.as_mut().unwrap().next;
        }
        result
    }
    pub fn add_two_numbers_self(&self) -> LinkedList {
      let mut list_01 = &self.l1;
      let mut list_02 = &self.l2;
      let mut addition = 0;
      let mut result = None;
      let mut current = &mut result;

      while list_01.is_some() || list_02.is_some() || addition != 0 {
        let mut result_temp = addition;
        
        if let Some(node) = list_01 {
          result_temp += node.val;
          list_01 = &node.next;
        }
        if let Some(node) = list_02 {
          result_temp += node.val;
          list_02 = &node.next;
        }
        addition = result_temp / 10;
        *current = Some(Box::new(ListNode::new(result_temp % 10)));
        current = &mut current.as_mut().unwrap().next;
      }
      result
  }
}

let result = Solution{l1: vec_to_ll(input_01), l2: vec_to_ll(input_02)}
.add_two_numbers_self();
println!("{:?}", result);
}