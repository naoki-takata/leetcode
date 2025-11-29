// Find the Duplicate Number
// 配列内の重複した数を検出する（配列を変更せず、定数空間のみ使用）

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Floyd's Cycle Detection Algorithm (ウサギとカメのアルゴリズム)
        // 配列のインデックスをノード、配列の値を次のノードへのポインタとして扱う
        // 重複があるとサイクルが形成され、サイクルの入り口が重複した数になる
        
        // フェーズ1: サイクルを検出
        let mut slow = nums[0] as usize;
        let mut fast = nums[0] as usize;
        
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        
        // フェーズ2: サイクルの入り口を見つける
        // スローポインタを先頭に戻し、両方のポインタを1つずつ進める
        slow = nums[0] as usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        
        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    #[test]
    fn test_example2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    #[test]
    fn test_example3() {
        let nums = vec![3, 3, 3, 3, 3];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }

    #[test]
    fn test_small_array() {
        let nums = vec![1, 1];
        assert_eq!(Solution::find_duplicate(nums), 1);
    }

    #[test]
    fn test_larger_array() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 5];
        assert_eq!(Solution::find_duplicate(nums), 5);
    }
}

