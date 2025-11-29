// Peeking Iterator
// 既存のイテレータにpeek操作を追加するイテレータ

// LeetCode提出用の実装
// 注意: LeetCode環境では、Iteratorトレイトが提供される場合があります
// その場合は、提供されるIteratorトレイトに合わせて実装を調整してください

// 標準的な実装（Vecベース - LeetCode提出用）
struct PeekingIterator {
    vec: Vec<i32>,
    index: usize,
    peeked: Option<i32>,
}

impl PeekingIterator {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            vec: nums,
            index: 0,
            peeked: None,
        }
    }

    fn peek(&mut self) -> i32 {
        // まだ先読みしていない場合は先読みする
        if self.peeked.is_none() {
            if self.index < self.vec.len() {
                self.peeked = Some(self.vec[self.index]);
            }
        }
        // 問題の制約により、peekは常に有効な値を持つことが保証されている
        self.peeked.unwrap()
    }

    fn next(&mut self) -> i32 {
        // 先読みした値があればそれを返し、なければベクトルから取得
        if let Some(value) = self.peeked.take() {
            self.index += 1;
            value
        } else {
            let value = self.vec[self.index];
            self.index += 1;
            value
        }
    }

    fn has_next(&self) -> bool {
        // 先読みした値があるか、まだ要素が残っているか
        self.peeked.is_some() || self.index < self.vec.len()
    }
}

// LeetCode環境でIteratorトレイトが提供される場合の実装例
// 以下のコードは参考用です。実際のLeetCode環境に合わせて調整してください
/*
// LeetCode提供のIteratorトレイト（仮定）
trait Iterator {
    fn next(&mut self) -> Option<i32>;
    fn has_next(&self) -> bool;
}

struct PeekingIterator {
    iter: Box<dyn Iterator>,
    peeked: Option<i32>,
}

impl PeekingIterator {
    fn new(iter: Box<dyn Iterator>) -> Self {
        Self {
            iter,
            peeked: None,
        }
    }

    fn peek(&mut self) -> i32 {
        if self.peeked.is_none() {
            if self.iter.has_next() {
                self.peeked = self.iter.next();
            }
        }
        self.peeked.unwrap()
    }

    fn next(&mut self) -> i32 {
        if let Some(value) = self.peeked.take() {
            value
        } else {
            self.iter.next().unwrap()
        }
    }

    fn has_next(&self) -> bool {
        self.peeked.is_some() || self.iter.has_next()
    }
}
*/

// より汎用的な実装（ジェネリック版）
struct PeekingIteratorGeneric<T, I: Iterator<Item = T>> {
    iter: I,
    peeked: Option<T>,
}

impl<T, I: Iterator<Item = T>> PeekingIteratorGeneric<T, I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            peeked: None,
        }
    }

    fn peek(&mut self) -> Option<&T> {
        if self.peeked.is_none() {
            self.peeked = self.iter.next();
        }
        self.peeked.as_ref()
    }

    fn next(&mut self) -> Option<T> {
        self.peeked.take().or_else(|| self.iter.next())
    }

    fn has_next(&mut self) -> bool {
        if self.peeked.is_some() {
            return true;
        }
        // 次の要素を先読みして確認
        self.peeked = self.iter.next();
        self.peeked.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 2, 3];
        let mut iter = PeekingIterator::new(nums);
        
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.peek(), 2);
        assert_eq!(iter.next(), 2);
        assert_eq!(iter.next(), 3);
        assert_eq!(iter.has_next(), false);
    }

    #[test]
    fn test_peek_multiple_times() {
        let nums = vec![1, 2, 3];
        let mut iter = PeekingIterator::new(nums);
        
        assert_eq!(iter.peek(), 1);
        assert_eq!(iter.peek(), 1); // 複数回peekしても同じ値
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.peek(), 2);
        assert_eq!(iter.next(), 2);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        let mut iter = PeekingIterator::new(nums);
        
        assert_eq!(iter.peek(), 42);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 42);
        assert_eq!(iter.has_next(), false);
    }

    #[test]
    fn test_generic_version() {
        let nums = vec![1, 2, 3];
        let mut iter = PeekingIteratorGeneric::new(nums.into_iter());
        
        assert_eq!(iter.peek(), Some(&1));
        assert_eq!(iter.peek(), Some(&1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.peek(), Some(&2));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.has_next(), false);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_generic_with_strings() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut iter = PeekingIteratorGeneric::new(words.into_iter());
        
        assert_eq!(iter.peek(), Some(&"hello".to_string()));
        assert_eq!(iter.next(), Some("hello".to_string()));
        assert_eq!(iter.peek(), Some(&"world".to_string()));
        assert_eq!(iter.next(), Some("world".to_string()));
        assert_eq!(iter.has_next(), false);
    }
}

