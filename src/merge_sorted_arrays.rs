/// Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as
/// one sorted array.
/// Note:
/// The number of elements initialized in nums1 and nums2 are m and n respectively.
/// You may assume that nums1 has enough space (size that is greater or equal to m + n) to hold additional elements from nums2.
///
/// Example:
/// Input:
/// nums1 = [1,2,3,0,0,0], m = 3
/// nums2 = [2,5,6],       n = 3
///
/// Output: [1,2,2,3,5,6]
/// 
/// P.S.:
/// Edge cases for you to suffer:
/// [0], 0, [1], 1 should eval [1]
/// [0], 0, [], 0 should eval [0]
/// [1], 1, [], 0 should not panic
/// [-1,0,0,3,3,3,0,0,0], 6, [1,2,2], 3 should eval [-1,0,0,1,2,2,3,3,3]
/// [-1,0,1,2,3,4,0,0,0], 6, [-2,0,1], 3 should eval [-2,-1,0,0,1,1,2,3,4]
/// [0,0,3,0,0,0,0,0,0], 3, [-1,1,1,1,2,3], 6 should eval [-1,0,0,1,1,1,2,3,3]

pub fn merge_two_arrays(input_01: &mut Vec<i32>, input_02: &mut Vec<i32>) {

struct Solution;

impl Solution {

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        
        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }

    pub fn merge_old(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let full_len = nums1.len();
        if nums1.is_empty() || nums2.is_empty() {return ();}
        else if m == 0 && n == 0 {return ();}
        else if m == 0 && n != 0 {std::mem::swap(nums1, nums2); return ();}
        else if n == 0 {return ();}

        if nums1[0] >= 0 {
            nums1.retain(|&x| x != 0);
        }
        else {
            let mut index = nums1.len() - 1;
            loop {
                if nums1[index] == 0 {
                    nums1.remove(index);
                    index -= 1;
                }
                else {break;}
            }
        }
        
        if nums2[0] >= 0 {
            nums2.retain(|&x| x != 0);
        }
        else {
            let mut index = nums2.len() - 1;
            loop {
                if nums2[index] == 0 {
                    nums2.remove(index);
                    index -= 1;
                }
                else {break;}
            }
        }

        if nums1[nums1.len() - 1] < nums2[0] {
            for item in nums2.into_iter() {
                nums1.push(*item);
            }
        }
        else if nums1[0] > nums2[nums2.len() - 1] {
            for item in nums1.into_iter() {
                nums2.push(*item);
            }
            std::mem::swap(nums1, nums2);
        }
        else {
            nums1.extend(nums2.iter());
            nums1.sort();
        }

        if nums1.len() < full_len {
            let mut len_fix = nums1.len();
            loop {
                if len_fix != full_len {
                    nums1.push(0i32);
                    len_fix += 1;
                }
                else {break;}
            }
        }
        // println!("{:?}", nums1);
        // println!("{:?}", nums2);
    }
}
let m = input_01.len() as i32;
let n = input_02.len() as i32;
Solution::merge(input_01, m, input_02, n);
}