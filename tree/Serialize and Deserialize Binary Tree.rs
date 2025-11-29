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
use std::collections::VecDeque;

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    /// 二分木を文字列にシリアライズする（LeetCode形式）
    /// 例: "[1,2,3,null,null,4,5]"
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "[]".to_string();
        }

        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while let Some(node_opt) = queue.pop_front() {
            match node_opt {
                Some(node) => {
                    let node_ref = node.borrow();
                    result.push(node_ref.val.to_string());
                    queue.push_back(node_ref.left.clone());
                    queue.push_back(node_ref.right.clone());
                }
                None => {
                    result.push("null".to_string());
                }
            }
        }

        // 末尾のnullを削除
        while let Some(last) = result.last() {
            if last == "null" {
                result.pop();
            } else {
                break;
            }
        }

        format!("[{}]", result.join(","))
    }

    /// 文字列から二分木をデシリアライズする
    /// 例: "[1,2,3,null,null,4,5]"
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        // 角括弧を削除して、カンマで分割
        let trimmed = data.trim();
        if trimmed == "[]" || trimmed.is_empty() {
            return None;
        }

        let content = &trimmed[1..trimmed.len() - 1]; // 角括弧を削除
        if content.is_empty() {
            return None;
        }

        let values: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        if values.is_empty() || values[0].is_empty() {
            return None;
        }

        // 最初のノードを作成
        let root_val = values[0].parse::<i32>().ok()?;
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while let Some(node) = queue.pop_front() {
            // 左の子
            if i < values.len() && values[i] != "null" {
                if let Ok(val) = values[i].parse::<i32>() {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                }
            }
            i += 1;

            // 右の子
            if i < values.len() && values[i] != "null" {
                if let Ok(val) = values[i].parse::<i32>() {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                }
            }
            i += 1;
        }

        Some(root)
    }
}

// 使用例（テスト用）
// fn main() {
//     let codec = Codec::new();
//     
//     // 例1: [1,2,3,null,null,4,5]
//     let root = codec.deserialize("[1,2,3,null,null,4,5]".to_string());
//     let serialized = codec.serialize(root);
//     println!("{}", serialized);
//     
//     // 例2: []
//     let root2 = codec.deserialize("[]".to_string());
//     let serialized2 = codec.serialize(root2);
//     println!("{}", serialized2);
// }

