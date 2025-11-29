impl Solution {
    /// O(n log(n)) の時間計算量で最長増加部分列の長さを求める
    /// 二分探索を使用して、各長さの増加部分列の最小末尾要素を保持
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        // tails[i] = 長さ i+1 の増加部分列の最小末尾要素
        let mut tails = Vec::new();
        
        for &num in nums.iter() {
            // 現在の要素を配置する位置を二分探索で見つける
            // tails の中で num より大きい最初の要素の位置を見つける
            let pos = tails.binary_search(&num).unwrap_or_else(|x| x);
            
            if pos == tails.len() {
                // num が tails のすべての要素より大きい場合、新しい長さの部分列を作成
                tails.push(num);
            } else {
                // num より大きい要素を num で置き換える（より小さい末尾要素で更新）
                tails[pos] = num;
            }
        }
        
        tails.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]),
            4
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]),
            1
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(
            Solution::length_of_lis(vec![1]),
            1
        );
    }

    #[test]
    fn test_strictly_increasing() {
        assert_eq!(
            Solution::length_of_lis(vec![1, 2, 3, 4, 5]),
            5
        );
    }

    #[test]
    fn test_strictly_decreasing() {
        assert_eq!(
            Solution::length_of_lis(vec![5, 4, 3, 2, 1]),
            1
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(
            Solution::length_of_lis(vec![-10, -5, 0, 5, 10]),
            5
        );
    }

    #[test]
    fn test_mixed_positive_negative() {
        assert_eq!(
            Solution::length_of_lis(vec![0, -1, 2, -2, 3, -3, 4]),
            4
        );
    }
}

