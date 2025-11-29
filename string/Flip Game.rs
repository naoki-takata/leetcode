use std::collections::HashMap;

impl Solution {
    pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
        let mut result = Vec::new();
        let bytes = current_state.as_bytes();
        
        // 文字列を走査して、連続する "++" を見つける
        for i in 0..bytes.len().saturating_sub(1) {
            if bytes[i] == b'+' && bytes[i + 1] == b'+' {
                // "++" を "--" に置き換えた新しい文字列を作成
                let new_state = format!(
                    "{}{}{}",
                    &current_state[..i],
                    "--",
                    &current_state[i + 2..]
                );
                result.push(new_state);
            }
        }
        
        result
    }

    pub fn can_win(current_state: String) -> bool {
        let mut memo: HashMap<String, bool> = HashMap::new();
        Self::can_win_helper(current_state, &mut memo)
    }

    fn can_win_helper(current_state: String, memo: &mut HashMap<String, bool>) -> bool {
        // メモ化チェック
        if let Some(&result) = memo.get(&current_state) {
            return result;
        }

        // 可能な手をすべて取得
        let next_moves = Self::generate_possible_next_moves(current_state.clone());
        
        // 可能な手がない場合、現在のプレイヤーは負け
        if next_moves.is_empty() {
            memo.insert(current_state, false);
            return false;
        }

        // すべての可能な手を試す
        for next_state in next_moves {
            // 次の状態で相手が負ける（つまり、次の状態で開始プレイヤーが負ける）なら、
            // 現在のプレイヤーは勝てる
            if !Self::can_win_helper(next_state, memo) {
                memo.insert(current_state, true);
                return true;
            }
        }

        // すべての手を試しても勝てる手がない場合、負ける
        memo.insert(current_state, false);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::generate_possible_next_moves("++++".to_string());
        let mut expected = vec!["--++".to_string(), "+--+".to_string(), "++--".to_string()];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let result = Solution::generate_possible_next_moves("+".to_string());
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_no_plus() {
        let result = Solution::generate_possible_next_moves("--".to_string());
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_single_plus_pair() {
        let result = Solution::generate_possible_next_moves("++".to_string());
        assert_eq!(result, vec!["--".to_string()]);
    }

    #[test]
    fn test_mixed() {
        let result = Solution::generate_possible_next_moves("++--++".to_string());
        let mut expected = vec!["----++".to_string(), "++----".to_string()];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_long_string() {
        let result = Solution::generate_possible_next_moves("++++++".to_string());
        // 5つの可能な状態: "--++++", "+--+++", "++--++", "+++--+", "++++--"
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_can_win_example1() {
        // Example 1: "++++" -> true
        assert_eq!(Solution::can_win("++++".to_string()), true);
    }

    #[test]
    fn test_can_win_example2() {
        // Example 2: "+" -> false
        assert_eq!(Solution::can_win("+".to_string()), false);
    }

    #[test]
    fn test_can_win_no_moves() {
        // 可能な手がない場合
        assert_eq!(Solution::can_win("--".to_string()), false);
        assert_eq!(Solution::can_win("-".to_string()), false);
    }

    #[test]
    fn test_can_win_single_pair() {
        // "++" の場合、1手で勝てる
        assert_eq!(Solution::can_win("++".to_string()), true);
    }

    #[test]
    fn test_can_win_three_pairs() {
        // "++++++" の場合
        assert_eq!(Solution::can_win("++++++".to_string()), true);
    }
}

