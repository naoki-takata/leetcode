// Find Median from Data Stream
// データストリームから中央値を効率的に見つける問題
// 2つのヒープ（最大ヒープと最小ヒープ）を使用してO(log n)で要素を追加し、O(1)で中央値を取得

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    // 小さい半分の要素を保持する最大ヒープ
    // 最大値が中央値（または中央値の候補）になる
    max_heap: BinaryHeap<i32>,
    // 大きい半分の要素を保持する最小ヒープ（Reverseを使用して実現）
    // 最小値が中央値（または中央値の候補）になる
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        // 最初の要素はmax_heapに追加
        if self.max_heap.is_empty() {
            self.max_heap.push(num);
            return;
        }

        // max_heapの最大値と比較
        if num <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(Reverse(num));
        }

        // 2つのヒープのサイズのバランスを取る
        // max_heapのサイズがmin_heapより2以上大きい場合、調整
        if self.max_heap.len() > self.min_heap.len() + 1 {
            let max_val = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(max_val));
        }
        // min_heapのサイズがmax_heapより大きい場合、調整
        else if self.min_heap.len() > self.max_heap.len() {
            let Reverse(min_val) = self.min_heap.pop().unwrap();
            self.max_heap.push(min_val);
        }
    }

    fn find_median(&self) -> f64 {
        // 要素数が奇数の場合、max_heapの最大値が中央値
        if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as f64
        }
        // 要素数が偶数の場合、2つのヒープのトップの平均が中央値
        else {
            let max_val = *self.max_heap.peek().unwrap() as f64;
            let Reverse(min_val) = self.min_heap.peek().unwrap();
            let min_val = *min_val as f64;
            (max_val + min_val) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        assert!((median_finder.find_median() - 1.5).abs() < 1e-5);
        median_finder.add_num(3);
        assert!((median_finder.find_median() - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_single_element() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(5);
        assert!((median_finder.find_median() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_two_elements() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        assert!((median_finder.find_median() - 1.5).abs() < 1e-5);
    }

    #[test]
    fn test_three_elements() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        median_finder.add_num(3);
        assert!((median_finder.find_median() - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_four_elements() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        median_finder.add_num(3);
        median_finder.add_num(4);
        assert!((median_finder.find_median() - 2.5).abs() < 1e-5);
    }

    #[test]
    fn test_negative_numbers() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(-1);
        median_finder.add_num(-2);
        assert!((median_finder.find_median() - (-1.5)).abs() < 1e-5);
        median_finder.add_num(-3);
        assert!((median_finder.find_median() - (-2.0)).abs() < 1e-5);
    }

    #[test]
    fn test_mixed_numbers() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(-1);
        median_finder.add_num(0);
        median_finder.add_num(1);
        assert!((median_finder.find_median() - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_duplicate_numbers() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(1);
        median_finder.add_num(1);
        assert!((median_finder.find_median() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_large_numbers() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(100000);
        median_finder.add_num(200000);
        assert!((median_finder.find_median() - 150000.0).abs() < 1e-5);
    }

    #[test]
    fn test_sequential_add() {
        let mut median_finder = MedianFinder::new();
        for i in 1..=10 {
            median_finder.add_num(i);
        }
        // [1,2,3,4,5,6,7,8,9,10] の中央値は (5+6)/2 = 5.5
        assert!((median_finder.find_median() - 5.5).abs() < 1e-5);
    }

    #[test]
    fn test_reverse_sequential_add() {
        let mut median_finder = MedianFinder::new();
        for i in (1..=10).rev() {
            median_finder.add_num(i);
        }
        // 最終的に [1,2,3,4,5,6,7,8,9,10] と同じ順序になる
        assert!((median_finder.find_median() - 5.5).abs() < 1e-5);
    }
}

