pub struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());
        for i in nums {
            if !set.insert(i) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_1() {
    assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
}

#[test]
fn test_2() {
    assert_eq!(false, Solution::contains_duplicate(vec![1, 2, 3, 4]));
}

#[test]
fn test_3() {
    assert_eq!(
        true,
        Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
    );
}
