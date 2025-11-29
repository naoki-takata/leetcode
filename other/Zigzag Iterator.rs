// Zigzag Iterator
// 2つのベクトルの要素を交互に返すイテレータ

struct ZigzagIterator {
    v1: Vec<i32>,
    v2: Vec<i32>,
    index1: usize,
    index2: usize,
    current: usize, // 0: v1, 1: v2
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        Self {
            v1,
            v2,
            index1: 0,
            index2: 0,
            current: 0, // 最初はv1から始める
        }
    }

    fn next(&mut self) -> i32 {
        // 現在のベクトルから要素を取得
        if self.current == 0 {
            // v1から取得
            if self.index1 < self.v1.len() {
                let result = self.v1[self.index1];
                self.index1 += 1;
                // 次はv2に切り替え
                self.current = 1;
                return result;
            } else {
                // v1が終わっているのでv2から取得
                let result = self.v2[self.index2];
                self.index2 += 1;
                return result;
            }
        } else {
            // v2から取得
            if self.index2 < self.v2.len() {
                let result = self.v2[self.index2];
                self.index2 += 1;
                // 次はv1に切り替え
                self.current = 0;
                return result;
            } else {
                // v2が終わっているのでv1から取得
                let result = self.v1[self.index1];
                self.index1 += 1;
                return result;
            }
        }
    }

    fn has_next(&self) -> bool {
        self.index1 < self.v1.len() || self.index2 < self.v2.len()
    }
}

// フォローアップ: k個のベクトルに対応するバージョン
struct ZigzagIteratorK {
    vectors: Vec<Vec<i32>>,
    indices: Vec<usize>,
    current: usize, // 現在どのベクトルから要素を取るか
}

impl ZigzagIteratorK {
    fn new(vectors: Vec<Vec<i32>>) -> Self {
        let k = vectors.len();
        Self {
            vectors,
            indices: vec![0; k],
            current: 0,
        }
    }

    fn next(&mut self) -> i32 {
        let k = self.vectors.len();
        
        // 現在のベクトルから要素を取得しようとする
        // もし現在のベクトルが終わっていれば、次の有効なベクトルを探す
        let mut attempts = 0;
        while attempts < k {
            let vec_idx = self.current;
            if self.indices[vec_idx] < self.vectors[vec_idx].len() {
                let result = self.vectors[vec_idx][self.indices[vec_idx]];
                self.indices[vec_idx] += 1;
                // 次のベクトルに移動（循環）
                self.current = (self.current + 1) % k;
                return result;
            } else {
                // 現在のベクトルが終わっているので、次のベクトルに移動
                self.current = (self.current + 1) % k;
                attempts += 1;
            }
        }
        
        // ここには到達しないはず（has_nextでチェック済み）
        unreachable!()
    }

    fn has_next(&self) -> bool {
        let k = self.vectors.len();
        for i in 0..k {
            if self.indices[i] < self.vectors[i].len() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut iter = ZigzagIterator::new(vec![1, 2], vec![3, 4, 5, 6]);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next());
        }
        assert_eq!(result, vec![1, 3, 2, 4, 5, 6]);
    }

    #[test]
    fn test_example2() {
        let mut iter = ZigzagIterator::new(vec![1], vec![]);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next());
        }
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_example3() {
        let mut iter = ZigzagIterator::new(vec![], vec![1]);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next());
        }
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_followup() {
        let mut iter = ZigzagIteratorK::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9],
        ]);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next());
        }
        assert_eq!(result, vec![1, 4, 8, 2, 5, 9, 3, 6, 7]);
    }
}

