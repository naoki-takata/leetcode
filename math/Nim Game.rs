impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        // Nim Gameの最適戦略:
        // - n = 1, 2, 3: 先手が全て取れるので勝てる
        // - n = 4: 先手が1,2,3個取っても、後手が残りを取って勝つ
        // - n = 5, 6, 7: 先手が適切に取れば（5なら1個、6なら2個、7なら3個）、
        //   後手を4個の状況にできるので先手が勝てる
        // - n = 8: 先手が何を取っても、後手が5,6,7の状況にでき、それらは全て
        //   先手が勝てる状況なので後手が勝つ
        // 
        // パターン: n % 4 == 0 の場合、先手は負ける。それ以外は先手が勝てる。
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // n = 4: 先手は負ける
        assert_eq!(Solution::can_win_nim(4), false);
    }

    #[test]
    fn test_example2() {
        // n = 1: 先手が1個取って勝つ
        assert_eq!(Solution::can_win_nim(1), true);
    }

    #[test]
    fn test_example3() {
        // n = 2: 先手が2個取って勝つ
        assert_eq!(Solution::can_win_nim(2), true);
    }

    #[test]
    fn test_n3() {
        // n = 3: 先手が3個取って勝つ
        assert_eq!(Solution::can_win_nim(3), true);
    }

    #[test]
    fn test_n5() {
        // n = 5: 先手が1個取れば、後手は4個の状況になり負ける
        assert_eq!(Solution::can_win_nim(5), true);
    }

    #[test]
    fn test_n8() {
        // n = 8: 先手は負ける
        assert_eq!(Solution::can_win_nim(8), false);
    }

    #[test]
    fn test_n12() {
        // n = 12: 先手は負ける（4の倍数）
        assert_eq!(Solution::can_win_nim(12), false);
    }

    #[test]
    fn test_large_number() {
        // 大きな数でも正しく動作することを確認
        assert_eq!(Solution::can_win_nim(100), false); // 4の倍数
        assert_eq!(Solution::can_win_nim(101), true);  // 4の倍数でない
    }
}

