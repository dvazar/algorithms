use std::cmp::min;

/// https://en.wikipedia.org/wiki/Partition_problem
/// https://leetcode.com/problems/partition-equal-subset-sum/
/// Constraints:
///   1 <= nums.length <= 200
///   1 <= nums[i] <= 100
pub fn can_partition(nums: Vec<i32>) -> bool {
    let mut min_value = i32::MAX;
    let mut array_sum = 0;
    for num in nums.iter() {
        min_value = min(min_value, *num);
        array_sum += *num;
    }
    if nums.len() == 1 || array_sum % 2 == 1 {
        return false;
    }
    let target_sum = array_sum / 2;

    let mut found_sum = vec![false; target_sum as usize];

    for num in nums.into_iter() {
        for sum in (min_value..=target_sum).rev() {
            if (num == sum) || (num < sum && found_sum[(sum - num - 1) as usize]) {
                if sum == target_sum { return true }
                found_sum[(sum - 1) as usize] = true;
            }
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_problem_1() {
        assert_eq!(can_partition(vec![1, 5, 1, 11, 3, 7]), true);
    }

    #[test]
    fn test_partition_problem_2() {
        assert_eq!(can_partition(vec![1, 2, 5]), false);
    }

    #[test]
    fn test_partition_problem_3() {
        assert_eq!(can_partition(vec![1, 1, 1, 1]), true);
    }

    #[test]
    fn test_partition_problem_4() {
        assert_eq!(can_partition(vec![14, 9, 8, 4, 3, 2]), true);
    }

}
