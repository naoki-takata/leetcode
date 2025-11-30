// Make Sum Divisible by P
// 配列から最小の部分配列を削除して、残りの要素の合計がpで割り切れるようにする

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let p = p as i64;
        
        // 配列全体の合計を計算
        let total_sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let target_remainder = total_sum % p;
        
        // 既に割り切れる場合、何も削除する必要がない
        if target_remainder == 0 {
            return 0;
        }
        
        // 累積和の余りを記録するマップ
        // key: 余り, value: その余りが最後に出現したインデックス
        let mut remainder_map = HashMap::new();
        remainder_map.insert(0i64, -1); // 最初の位置（インデックス-1）での余りは0
        
        let mut current_remainder = 0i64;
        let mut min_length = n as i32;
        
        for (i, &num) in nums.iter().enumerate() {
            current_remainder = (current_remainder + num as i64) % p;
            
            // 現在の余りからtarget_remainderを引いた余りを探す
            // 部分配列[j+1..i]の合計の余りがtarget_remainderになるには、
            // prefix_sum[i+1] - prefix_sum[j+1] ≡ target_remainder (mod p)
            // つまり、prefix_sum[j+1] ≡ prefix_sum[i+1] - target_remainder (mod p)
            let mut need_remainder = (current_remainder - target_remainder) % p;
            if need_remainder < 0 {
                need_remainder += p;
            }
            
            // 以前にこの余りが出現した位置があれば、その間の部分配列を削除できる
            if let Some(&prev_index) = remainder_map.get(&need_remainder) {
                let subarray_length = i as i32 - prev_index;
                min_length = min_length.min(subarray_length);
            }
            
            // 現在の余りとインデックスを記録
            remainder_map.insert(current_remainder, i as i32);
        }
        
        // 配列全体を削除することはできないので、min_lengthがnと等しい場合は-1を返す
        if min_length == n as i32 {
            -1
        } else {
            min_length
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![3, 1, 4, 2];
        let p = 6;
        assert_eq!(Solution::min_subarray(nums, p), 1);
    }

    #[test]
    fn test_example2() {
        let nums = vec![6, 3, 5, 2];
        let p = 9;
        assert_eq!(Solution::min_subarray(nums, p), 2);
    }

    #[test]
    fn test_example3() {
        let nums = vec![1, 2, 3];
        let p = 3;
        assert_eq!(Solution::min_subarray(nums, p), 0);
    }

    #[test]
    fn test_impossible() {
        let nums = vec![1, 2, 3];
        let p = 7;
        // 合計は6、7で割った余りは6
        // 6 mod 7 = 6の部分配列を見つける必要があるが、不可能
        assert_eq!(Solution::min_subarray(nums, p), -1);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        let p = 1;
        // 合計は1、1で割り切れるので0
        assert_eq!(Solution::min_subarray(nums, p), 0);
    }

    #[test]
    fn test_single_element_remove() {
        let nums = vec![3];
        let p = 2;
        // 合計は3、2で割った余りは1
        // 1 mod 2 = 1の部分配列を見つける必要があるが、配列全体を削除できないので-1
        assert_eq!(Solution::min_subarray(nums, p), -1);
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![1000000000, 1000000000, 1000000000];
        let p = 3;
        // 合計は3000000000、3で割り切れるので0
        assert_eq!(Solution::min_subarray(nums, p), 0);
    }
}

