// Best Meeting Point
// m x n のバイナリグリッドで、友人の家（1でマーク）から最小の総移動距離を求める
// マンハッタン距離を使用し、最適な会議地点を見つける
// 最適な会議地点は、x座標とy座標それぞれの中央値となる

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        // すべての友人の座標を収集
        let mut friends = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    friends.push((i as i32, j as i32));
                }
            }
        }
        
        // 友人が2人未満の場合は0を返す（問題の制約では少なくとも2人いることが保証されている）
        if friends.len() < 2 {
            return 0;
        }
        
        // x座標とy座標を別々に収集
        let mut x_coords: Vec<i32> = friends.iter().map(|&(x, _)| x).collect();
        let mut y_coords: Vec<i32> = friends.iter().map(|&(_, y)| y).collect();
        
        // ソートして中央値を取得
        x_coords.sort();
        y_coords.sort();
        
        // 中央値の計算
        // 要素数が奇数の場合: 中央の要素
        // 要素数が偶数の場合: 中央の2つの要素のどちらでも同じ距離の合計になる
        let median_x = x_coords[x_coords.len() / 2];
        let median_y = y_coords[y_coords.len() / 2];
        
        // 各友人から中央値までのマンハッタン距離の合計を計算
        let mut total_distance = 0;
        for &(x, y) in &friends {
            total_distance += (x - median_x).abs() + (y - median_y).abs();
        }
        
        total_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        assert_eq!(Solution::min_total_distance(grid), 6);
    }

    #[test]
    fn test_example2() {
        let grid = vec![
            vec![1, 1],
        ];
        assert_eq!(Solution::min_total_distance(grid), 1);
    }

    #[test]
    fn test_two_friends() {
        let grid = vec![
            vec![1, 0, 1],
        ];
        // 友人は (0,0) と (0,2)
        // 中央値は x=0, y=1
        // 距離: |0-0|+|0-1| + |0-0|+|2-1| = 1 + 1 = 2
        assert_eq!(Solution::min_total_distance(grid), 2);
    }

    #[test]
    fn test_three_friends_horizontal() {
        let grid = vec![
            vec![1, 1, 1],
        ];
        // 友人は (0,0), (0,1), (0,2)
        // 中央値は x=0, y=1
        // 距離: |0-0|+|0-1| + |0-0|+|1-1| + |0-0|+|2-1| = 1 + 0 + 1 = 2
        assert_eq!(Solution::min_total_distance(grid), 2);
    }

    #[test]
    fn test_three_friends_vertical() {
        let grid = vec![
            vec![1],
            vec![1],
            vec![1],
        ];
        // 友人は (0,0), (1,0), (2,0)
        // 中央値は x=1, y=0
        // 距離: |0-1|+|0-0| + |1-1|+|0-0| + |2-1|+|0-0| = 1 + 0 + 1 = 2
        assert_eq!(Solution::min_total_distance(grid), 2);
    }

    #[test]
    fn test_four_friends_square() {
        let grid = vec![
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1],
        ];
        // 友人は (0,0), (0,2), (2,0), (2,2)
        // 中央値は x=1, y=1
        // 距離: 各友人から (1,1) までの距離は 2
        // 合計: 2 + 2 + 2 + 2 = 8
        assert_eq!(Solution::min_total_distance(grid), 8);
    }

    #[test]
    fn test_five_friends() {
        let grid = vec![
            vec![1, 0, 1, 0, 1],
        ];
        // 友人は (0,0), (0,2), (0,4)
        // 中央値は x=0, y=2
        // 距離: |0-0|+|0-2| + |0-0|+|2-2| + |0-0|+|4-2| = 2 + 0 + 2 = 4
        assert_eq!(Solution::min_total_distance(grid), 4);
    }

    #[test]
    fn test_large_grid() {
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 0, 0],
        ];
        // 友人は (1,1), (1,3), (3,1), (3,3)
        // 中央値は x=2, y=2
        // 距離: 各友人から (2,2) までの距離は 2
        // 合計: 2 + 2 + 2 + 2 = 8
        assert_eq!(Solution::min_total_distance(grid), 8);
    }
}

