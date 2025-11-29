// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_length = 0;
        Self::dfs(&root, None, 0, &mut max_length);
        max_length
    }
    
    /// DFSで各ノードを探索し、連続シーケンスの長さを計算する
    /// 
    /// # 引数
    /// - `node`: 現在のノード
    /// - `parent_val`: 親ノードの値（Noneの場合はルートノード）
    /// - `current_length`: 現在の連続シーケンスの長さ
    /// - `max_length`: これまでに見つかった最大の連続シーケンスの長さ
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        parent_val: Option<i32>,
        current_length: i32,
        max_length: &mut i32,
    ) {
        if let Some(n) = node {
            let node_ref = n.borrow();
            let val = node_ref.val;
            
            // 親ノードが存在し、現在のノードの値が親ノードの値+1である場合
            // 連続シーケンスを継続
            let new_length = if let Some(pv) = parent_val {
                if val == pv + 1 {
                    current_length + 1
                } else {
                    // 連続シーケンスが途切れた場合、新しいシーケンスを開始
                    1
                }
            } else {
                // ルートノードの場合、長さ1で開始
                1
            };
            
            // 最大長を更新
            *max_length = (*max_length).max(new_length);
            
            // 左の子を探索（現在のノードの値を親として渡す）
            Self::dfs(&node_ref.left, Some(val), new_length, max_length);
            // 右の子を探索（現在のノードの値を親として渡す）
            Self::dfs(&node_ref.right, Some(val), new_length, max_length);
        }
    }
}

