use std::collections::VecDeque;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        if rooms.is_empty() || rooms[0].is_empty() {
            return;
        }
        
        let m = rooms.len();
        let n = rooms[0].len();
        let inf = i32::MAX;
        
        // すべてのゲート（値が0のセル）をキューに追加
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if rooms[i][j] == 0 {
                    queue.push_back((i, j));
                }
            }
        }
        
        // 4方向への移動（上、右、下、左）
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        
        // BFSを実行
        while let Some((row, col)) = queue.pop_front() {
            let current_distance = rooms[row][col];
            
            // 4方向を探索
            for (dr, dc) in &directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                
                // 境界チェック
                if new_row < 0 || new_row >= m as i32 || new_col < 0 || new_col >= n as i32 {
                    continue;
                }
                
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                // 空の部屋（INF）の場合のみ更新
                if rooms[new_row][new_col] == inf {
                    rooms[new_row][new_col] = current_distance + 1;
                    queue.push_back((new_row, new_col));
                }
            }
        }
    }
}

