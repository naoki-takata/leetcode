use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        // 文字列が有効かどうかをチェックする関数
        fn is_valid(s: &str) -> bool {
            let mut count = 0;
            for ch in s.chars() {
                match ch {
                    '(' => count += 1,
                    ')' => {
                        count -= 1;
                        if count < 0 {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
            count == 0
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut found = false;

        queue.push_back(s.clone());
        visited.insert(s.clone());

        while !queue.is_empty() {
            let size = queue.len();

            // 現在のレベル（同じ削除数）のすべての文字列を処理
            for _ in 0..size {
                let current = queue.pop_front().unwrap();

                // 有効な文字列が見つかった場合
                if is_valid(&current) {
                    result.push(current.clone());
                    found = true;
                }

                // 既に有効な文字列が見つかっている場合、次のレベルには進まない
                if found {
                    continue;
                }

                // 各位置の括弧を削除して新しい文字列を生成
                for (i, ch) in current.char_indices() {
                    // 括弧以外はスキップ
                    if ch != '(' && ch != ')' {
                        continue;
                    }

                    // 連続する同じ括弧をスキップ（重複を避けるため）
                    if i > 0 {
                        let prev_ch = current.chars().nth(i - 1).unwrap();
                        if ch == prev_ch {
                            continue;
                        }
                    }

                    // i番目の文字を削除した新しい文字列を作成
                    let mut new_str = current.clone();
                    new_str.remove(i);

                    // まだ訪問していない文字列の場合、キューに追加
                    if !visited.contains(&new_str) {
                        visited.insert(new_str.clone());
                        queue.push_back(new_str);
                    }
                }
            }

            // 有効な文字列が見つかった場合、処理を終了
            if found {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::remove_invalid_parentheses("()())()".to_string());
        let mut result_set: HashSet<String> = result.into_iter().collect();
        let expected: HashSet<String> = ["(())()".to_string(), "()()()".to_string()]
            .iter()
            .cloned()
            .collect();
        assert_eq!(result_set, expected);
    }

    #[test]
    fn test_example2() {
        let result = Solution::remove_invalid_parentheses("(a)())()".to_string());
        let mut result_set: HashSet<String> = result.into_iter().collect();
        let expected: HashSet<String> = ["(a())()".to_string(), "(a)()()".to_string()]
            .iter()
            .cloned()
            .collect();
        assert_eq!(result_set, expected);
    }

    #[test]
    fn test_example3() {
        let result = Solution::remove_invalid_parentheses(")(".to_string());
        let mut result_set: HashSet<String> = result.into_iter().collect();
        let expected: HashSet<String> = ["".to_string()].iter().cloned().collect();
        assert_eq!(result_set, expected);
    }

    #[test]
    fn test_single_invalid() {
        let result = Solution::remove_invalid_parentheses("()".to_string());
        let mut result_set: HashSet<String> = result.into_iter().collect();
        let expected: HashSet<String> = ["()".to_string()].iter().cloned().collect();
        assert_eq!(result_set, expected);
    }

    #[test]
    fn test_all_invalid() {
        let result = Solution::remove_invalid_parentheses("))".to_string());
        let mut result_set: HashSet<String> = result.into_iter().collect();
        let expected: HashSet<String> = ["".to_string()].iter().cloned().collect();
        assert_eq!(result_set, expected);
    }
}

