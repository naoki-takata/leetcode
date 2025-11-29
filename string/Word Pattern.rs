use std::collections::HashMap;

impl Solution {
    pub fn word_pattern_match(pattern: String, s: String) -> bool {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let s_bytes = s.as_bytes();
        
        // バックトラッキングでマッピングを探索
        Self::backtrack(
            &pattern_chars,
            s_bytes,
            0,
            0,
            &mut HashMap::new(),
            &mut HashMap::new(),
        )
    }
    
    fn backtrack(
        pattern: &[char],
        s: &[u8],
        pattern_idx: usize,
        s_idx: usize,
        char_to_str: &mut HashMap<char, Vec<u8>>,
        str_to_char: &mut HashMap<Vec<u8>, char>,
    ) -> bool {
        // パターンと文字列の両方が完全に消費された場合、成功
        if pattern_idx == pattern.len() && s_idx == s.len() {
            return true;
        }
        
        // どちらかが先に終わった場合、失敗
        if pattern_idx >= pattern.len() || s_idx >= s.len() {
            return false;
        }
        
        let ch = pattern[pattern_idx];
        
        // 既存のマッピングが存在する場合
        if let Some(mapped_str) = char_to_str.get(&ch) {
            // マッピングされた文字列が残りの文字列と一致するか確認
            if s_idx + mapped_str.len() <= s.len() 
                && &s[s_idx..s_idx + mapped_str.len()] == mapped_str.as_slice() {
                // 一致する場合、次のパターン文字へ進む
                return Self::backtrack(
                    pattern,
                    s,
                    pattern_idx + 1,
                    s_idx + mapped_str.len(),
                    char_to_str,
                    str_to_char,
                );
            } else {
                // 一致しない場合、失敗
                return false;
            }
        }
        
        // 新しいマッピングを試す
        // 現在の位置から、可能なすべての文字列長を試す
        for len in 1..=s.len() - s_idx {
            let candidate = s[s_idx..s_idx + len].to_vec();
            
            // 全単射の条件をチェック
            // この文字列が既に別の文字にマッピングされている場合はスキップ
            if str_to_char.contains_key(&candidate) {
                continue;
            }
            
            // 新しいマッピングを追加
            char_to_str.insert(ch, candidate.clone());
            str_to_char.insert(candidate.clone(), ch);
            
            // 再帰的に次のパターン文字を試す
            if Self::backtrack(
                pattern,
                s,
                pattern_idx + 1,
                s_idx + len,
                char_to_str,
                str_to_char,
            ) {
                return true;
            }
            
            // バックトラッキング：マッピングを元に戻す
            char_to_str.remove(&ch);
            str_to_char.remove(&candidate);
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::word_pattern_match("abab".to_string(), "redblueredblue".to_string()),
            true
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::word_pattern_match("aaaa".to_string(), "asdasdasdasd".to_string()),
            true
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::word_pattern_match("aabb".to_string(), "xyzabcxzyabc".to_string()),
            false
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            Solution::word_pattern_match("a".to_string(), "dog".to_string()),
            true
        );
    }

    #[test]
    fn test_single_char_repeated() {
        assert_eq!(
            Solution::word_pattern_match("aa".to_string(), "dogdog".to_string()),
            true
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            Solution::word_pattern_match("ab".to_string(), "aa".to_string()),
            false
        );
    }
}

