fn main() {
    let mut tree = BinaryTree::new_root(0_i32);
    let nums = vec![-2, -6, 12, 14, 3, -24, 8];
    for idx in nums {
        tree.add(idx);
    }

    assert!(tree.search(-6));
    assert!(tree.search(-24));
    assert!(tree.search(8));
    assert!(tree.search(0));
    assert!(!tree.search(101));
    assert!(!tree.search(-12));
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new_root(value: T) -> BinaryTree<T> {
        BinaryTree::NonEmpty(Box::new(TreeNode { element: value,
                                                 left: BinaryTree::Empty,
                                                 right: BinaryTree::Empty }))
    }

    fn add (&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode { element: value,
                                                                 left: BinaryTree::Empty,
                                                                 right: BinaryTree::Empty }))
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            },
        }
    }

    fn search(&self, value: T) -> bool {
        match *self {
            BinaryTree::Empty => {
                return false;
            },
            BinaryTree::NonEmpty(ref node) => {
                if value == node.element {
                    return true;
                } else if value <= node.element {
                    return node.left.search(value);
                } else {
                    return node.right.search(value);
                }
            }
        }
    }
}
