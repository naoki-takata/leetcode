// Game of Life
// 2Dグリッド上のセルオートマトンの次の状態を計算する
// インプレースで更新する必要があるため、一時的なマーキングを使用

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        
        // 8方向の隣接セル（上下左右と4つの対角）
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];
        
        // 第1パス: 各セルの次の状態を一時的な値でマーク
        for i in 0..m {
            for j in 0..n {
                let mut live_neighbors = 0;
                
                // 8方向の隣接セルをチェック
                for (di, dj) in directions.iter() {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        let neighbor_value = board[ni as usize][nj as usize];
                        // 元の値が1、または-1（元は1だったが死ぬことになる）の場合
                        if neighbor_value == 1 || neighbor_value == -1 {
                            live_neighbors += 1;
                        }
                    }
                }
                
                // ルールに基づいて次の状態を決定
                if board[i][j] == 1 {
                    // 生きているセル
                    if live_neighbors < 2 || live_neighbors > 3 {
                        // 死ぬ: -1としてマーク（元は1）
                        board[i][j] = -1;
                    }
                    // 2または3の隣接セルがある場合は生き続ける（1のまま）
                } else {
                    // 死んでいるセル
                    if live_neighbors == 3 {
                        // 生きる: 2としてマーク（元は0）
                        board[i][j] = 2;
                    }
                    // それ以外は死んだまま（0のまま）
                }
            }
        }
        
        // 第2パス: 一時的なマークを実際の値に変換
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == -1 {
                    board[i][j] = 0; // 死んだ
                } else if board[i][j] == 2 {
                    board[i][j] = 1; // 生き返った
                }
                // 1と0はそのまま
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut board = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![
            vec![0, 0, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![0, 1, 0],
        ]);
    }

    #[test]
    fn test_example2() {
        let mut board = vec![
            vec![1, 1],
            vec![1, 0],
        ];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![
            vec![1, 1],
            vec![1, 1],
        ]);
    }

    #[test]
    fn test_all_dead() {
        let mut board = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let expected = board.clone();
        Solution::game_of_life(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_all_alive() {
        let mut board = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ];
        Solution::game_of_life(&mut board);
        // すべてのセルが8つの隣接セルを持つため、過密で死ぬ
        assert_eq!(board, vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
    }

    #[test]
    fn test_single_cell() {
        let mut board = vec![vec![1]];
        Solution::game_of_life(&mut board);
        // 隣接セルが0なので死ぬ
        assert_eq!(board, vec![vec![0]]);
    }

    #[test]
    fn test_reproduction() {
        // 死んでいるセルが3つの隣接セルで生き返る
        let mut board = vec![
            vec![1, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ];
        Solution::game_of_life(&mut board);
        // (1,1)の位置が3つの隣接セルで生き返る
        assert_eq!(board[1][1], 1);
    }

    #[test]
    fn test_underpopulation() {
        // 生きているセルが1つだけ（隣接セルが1未満）で死ぬ
        let mut board = vec![
            vec![1, 0],
            vec![0, 0],
        ];
        Solution::game_of_life(&mut board);
        assert_eq!(board[0][0], 0);
    }

    #[test]
    fn test_overpopulation() {
        // 生きているセルが4つ以上の隣接セルで死ぬ
        let mut board = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![0, 1, 0],
        ];
        Solution::game_of_life(&mut board);
        // 中央のセル(1,1)は4つの隣接セルがあるので死ぬ
        assert_eq!(board[1][1], 0);
    }
}

