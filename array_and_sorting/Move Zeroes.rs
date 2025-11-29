// Move Zeroes
// 配列内のすべての0を末尾に移動させ、非ゼロ要素の相対的な順序を維持する

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut write_index = 0;
        
        // 非ゼロ要素を前から順に詰めていく
        for read_index in 0..nums.len() {
            if nums[read_index] != 0 {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }
        
        // 残りの部分を0で埋める
        for i in write_index..nums.len() {
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_example2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_no_zeros() {
        let mut nums = vec![1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_all_zeros() {
        let mut nums = vec![0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0]);
    }

    #[test]
    fn test_zeros_at_end() {
        let mut nums = vec![1, 2, 3, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 0, 0]);
    }

    #[test]
    fn test_zeros_at_beginning() {
        let mut nums = vec![0, 0, 1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 0, 0]);
    }

    #[test]
    fn test_mixed() {
        let mut nums = vec![0, 1, 0, 2, 0, 3, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 0, 0, 0, 0]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut nums = vec![0, -1, 0, -2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![-1, -2, 3, 0, 0]);
    }
}

