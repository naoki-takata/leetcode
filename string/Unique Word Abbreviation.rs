use std::collections::HashMap;
use std::collections::HashSet;

struct ValidWordAbbr {
    // 略語をキーとして、その略語を持つ単語の集合を値とするマップ
    abbr_to_words: HashMap<String, HashSet<String>>,
}

impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut abbr_to_words: HashMap<String, HashSet<String>> = HashMap::new();
        
        // 辞書の各単語について、略語を計算してマップに追加
        for word in dictionary {
            let abbr = Self::get_abbreviation(&word);
            abbr_to_words
                .entry(abbr)
                .or_insert_with(HashSet::new)
                .insert(word);
        }
        
        ValidWordAbbr { abbr_to_words }
    }
    
    // 単語の略語を生成するヘルパー関数
    // 例: "dog" -> "d1g", "internationalization" -> "i18n", "it" -> "it"
    fn get_abbreviation(word: &str) -> String {
        let len = word.len();
        if len <= 2 {
            // 2文字以下の場合は、そのまま返す
            word.to_string()
        } else {
            // 最初の文字 + 中間の文字数 + 最後の文字
            format!(
                "{}{}{}",
                &word[0..1],
                len - 2,
                &word[len - 1..]
            )
        }
    }
    
    fn is_unique(&self, word: String) -> bool {
        let abbr = Self::get_abbreviation(&word);
        
        // その略語を持つ単語の集合を取得
        if let Some(words) = self.abbr_to_words.get(&abbr) {
            // 略語を持つ単語が存在する場合
            // その集合にwordが含まれていて、かつ集合のサイズが1の場合のみtrue
            words.len() == 1 && words.contains(&word)
        } else {
            // 略語を持つ単語が存在しない場合はtrue
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let valid_word_abbr = ValidWordAbbr::new(vec![
            "deer".to_string(),
            "door".to_string(),
            "cake".to_string(),
            "card".to_string(),
        ]);
        
        assert_eq!(valid_word_abbr.is_unique("dear".to_string()), false);
        assert_eq!(valid_word_abbr.is_unique("cart".to_string()), true);
        assert_eq!(valid_word_abbr.is_unique("cane".to_string()), false);
        assert_eq!(valid_word_abbr.is_unique("make".to_string()), true);
        assert_eq!(valid_word_abbr.is_unique("cake".to_string()), true);
    }
    
    #[test]
    fn test_abbreviation() {
        assert_eq!(ValidWordAbbr::get_abbreviation("dog"), "d1g");
        assert_eq!(ValidWordAbbr::get_abbreviation("internationalization"), "i18n");
        assert_eq!(ValidWordAbbr::get_abbreviation("it"), "it");
        assert_eq!(ValidWordAbbr::get_abbreviation("a"), "a");
    }
    
    #[test]
    fn test_single_word() {
        let valid_word_abbr = ValidWordAbbr::new(vec!["hello".to_string()]);
        assert_eq!(valid_word_abbr.is_unique("hello".to_string()), true);
        assert_eq!(valid_word_abbr.is_unique("h3o".to_string()), false);
    }
    
    #[test]
    fn test_duplicate_words() {
        let valid_word_abbr = ValidWordAbbr::new(vec![
            "cake".to_string(),
            "cake".to_string(),
        ]);
        // 同じ単語が2回ある場合、集合には1つだけ保存される
        assert_eq!(valid_word_abbr.is_unique("cake".to_string()), true);
    }
}

