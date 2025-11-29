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
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.as_ref()?.borrow().val;
        let mut successor: Option<Rc<RefCell<TreeNode>>> = None;
        let mut current = root;

        // ルートからpまで探索しながら、pより大きい最小のノードを見つける
        while let Some(node) = current {
            let node_val = node.borrow().val;

            if node_val > p_val {
                // 現在のノードはpより大きいので、候補として記録
                successor = Some(node.clone());
                // より小さい値（より近い値）を探すため、左の子へ進む
                current = node.borrow().left.clone();
            } else {
                // 現在のノードはp以下なので、右の子へ進む
                current = node.borrow().right.clone();
            }
        }

        successor
    }
}

