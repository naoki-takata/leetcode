use std::collections::HashMap;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let secret_chars: Vec<char> = secret.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();
        
        // まず、bulls（同じ位置で同じ数字）を数える
        let mut bulls = 0;
        let mut secret_count = HashMap::new();
        let mut guess_count = HashMap::new();
        
        for i in 0..secret_chars.len() {
            if secret_chars[i] == guess_chars[i] {
                // 同じ位置で同じ数字の場合、bullsとしてカウント
                bulls += 1;
            } else {
                // bullsとして使われなかった数字の出現回数を数える
                *secret_count.entry(secret_chars[i]).or_insert(0) += 1;
                *guess_count.entry(guess_chars[i]).or_insert(0) += 1;
            }
        }
        
        // cowsを計算：各数字について、secretとguessの最小出現回数の合計
        let mut cows = 0;
        for (digit, &count) in secret_count.iter() {
            if let Some(&guess_cnt) = guess_count.get(digit) {
                cows += count.min(guess_cnt);
            }
        }
        
        format!("{}A{}B", bulls, cows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::get_hint("1807".to_string(), "7810".to_string()),
            "1A3B"
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::get_hint("1123".to_string(), "0111".to_string()),
            "1A1B"
        );
    }

    #[test]
    fn test_all_bulls() {
        assert_eq!(
            Solution::get_hint("1234".to_string(), "1234".to_string()),
            "4A0B"
        );
    }

    #[test]
    fn test_all_cows() {
        assert_eq!(
            Solution::get_hint("1234".to_string(), "4321".to_string()),
            "0A4B"
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            Solution::get_hint("1234".to_string(), "5678".to_string()),
            "0A0B"
        );
    }

    #[test]
    fn test_duplicate_digits() {
        assert_eq!(
            Solution::get_hint("1122".to_string(), "2211".to_string()),
            "0A4B"
        );
    }
}

