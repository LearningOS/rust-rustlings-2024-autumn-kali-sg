/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if self.root.is_none(){
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        let current_node = self.root.as_mut().unwrap();  // 树的根节点已经判断过了，所以可以安全的使用unwrap()
        if current_node.value > value {
            match current_node.left.as_mut() {
                Some(node) => node.insert(value),        // 调用树本身的插入方法
                None => current_node.left = Some(Box::new(TreeNode::new(value))),
            }
        } else if current_node.value == value {
            return;
        } else {
            match current_node.right.as_mut() {
                Some(node) => node.insert(value),
                None => current_node.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut next_node = &self.root;    // Option没有copy，所以这里要使用&
        while true {
            match next_node{
                Some(node) => {
                    if node.value == value {
                        return true;
                    } else if node.value > value {
                        next_node = &node.left;                        // 与上面保持相同
                    } else {
                        next_node = &node.right;
                    }
                }
                None => break ,
            }
        };
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if self.value > value{
            match self.left.as_mut(){
                Some(left) => left.insert(value),      // 递归遍历左子树
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else if self.value == value{                                        // 值已经存在就不在插入
            return;
        } else {
            match self.right.as_mut(){
                Some(right) => right.insert(value),    // 递归遍历右子树
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


