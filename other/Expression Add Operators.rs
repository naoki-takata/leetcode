// Expression Add Operators
// 数字の文字列に演算子を挿入して、ターゲット値に評価される式をすべて見つける

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        let num_chars: Vec<char> = num.chars().collect();
        
        Self::backtrack(
            &num_chars,
            target as i64,
            0,
            0,
            0,
            String::new(),
            &mut result,
        );
        
        result
    }
    
    fn backtrack(
        num: &[char],
        target: i64,
        index: usize,
        current_val: i64,
        prev_val: i64,
        expression: String,
        result: &mut Vec<String>,
    ) {
        // すべての数字を使い切った場合
        if index == num.len() {
            if current_val == target {
                result.push(expression);
            }
            return;
        }
        
        // 現在の位置から、可能なすべての数値を作成
        let mut current_num = 0i64;
        let mut num_str = String::new();
        
        for i in index..num.len() {
            // 先頭ゼロのチェック（先頭が0で、かつ複数桁の場合は無効）
            if i > index && num[index] == '0' {
                break;
            }
            
            // 数値を構築
            current_num = current_num * 10 + (num[i] as u8 - b'0') as i64;
            num_str.push(num[i]);
            
            if index == 0 {
                // 最初の数値の場合、演算子は不要
                Self::backtrack(
                    num,
                    target,
                    i + 1,
                    current_num,
                    current_num,
                    num_str.clone(),
                    result,
                );
            } else {
                // 加算
                let new_expr = format!("{}+{}", expression, num_str);
                Self::backtrack(
                    num,
                    target,
                    i + 1,
                    current_val + current_num,
                    current_num,
                    new_expr,
                    result,
                );
                
                // 減算
                let new_expr = format!("{}-{}", expression, num_str);
                Self::backtrack(
                    num,
                    target,
                    i + 1,
                    current_val - current_num,
                    -current_num,
                    new_expr,
                    result,
                );
                
                // 乗算（優先順位を考慮）
                // 前の値を引いて、新しい値を掛けて、再度加算する
                let new_expr = format!("{}*{}", expression, num_str);
                let new_val = current_val - prev_val + prev_val * current_num;
                Self::backtrack(
                    num,
                    target,
                    i + 1,
                    new_val,
                    prev_val * current_num,
                    new_expr,
                    result,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::add_operators("123".to_string(), 6);
        let mut sorted_result: Vec<String> = result.into_iter().collect();
        sorted_result.sort();
        let mut expected = vec!["1+2+3".to_string(), "1*2*3".to_string()];
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_example2() {
        let result = Solution::add_operators("232".to_string(), 8);
        let mut sorted_result: Vec<String> = result.into_iter().collect();
        sorted_result.sort();
        let mut expected = vec!["2+3*2".to_string(), "2*3+2".to_string()];
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_example3() {
        let result = Solution::add_operators("3456237490".to_string(), 9191);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_leading_zero() {
        let result = Solution::add_operators("105".to_string(), 5);
        // "1*0+5" や "10-5" などが含まれる可能性がある
        // 先頭ゼロを持つ数値（例: "05"）は許可されない
        for expr in &result {
            // 式を検証（簡易チェック）
            assert!(!expr.contains("+05") && !expr.contains("-05") && !expr.contains("*05"));
        }
    }

    #[test]
    fn test_single_digit() {
        let result = Solution::add_operators("5".to_string(), 5);
        assert_eq!(result, vec!["5"]);
    }
}

