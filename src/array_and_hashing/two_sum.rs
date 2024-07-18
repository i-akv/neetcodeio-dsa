pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let remaining = target - num;
            match map.get(&remaining) {
                Some(&i) => return [i as i32, idx as i32].to_vec(),
                None => {
                    map.insert(*num, idx);
                }
            }
        }
        [].to_vec()
    }
}

#[test]
fn test_1() {
    let ans = Solution::two_sum([2, 7, 11, 15].to_vec(), 9);
    assert!([0, 1].to_vec() == ans || ans == [1, 0].to_vec());
}

#[test]
fn test_2() {
    let ans = Solution::two_sum([3, 2, 4].to_vec(), 6);
    assert!([1, 2].to_vec() == ans || ans == [2, 1].to_vec());
}

#[test]
fn test_3() {
    let ans = Solution::two_sum([3, 3].to_vec(), 6);
    assert!([0, 1].to_vec() == ans || ans == [1, 0].to_vec());
}
