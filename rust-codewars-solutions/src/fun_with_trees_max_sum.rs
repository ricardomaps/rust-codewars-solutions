#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn max_sum(tree: Option<&TreeNode>) -> i32 {
    match tree {
        None => 0,
        Some(node) => match node.left.as_ref().xor(node.right.as_ref()) {
            None => {
                let left_sum = max_sum(node.left.as_deref());
                let right_sum = max_sum(node.right.as_deref());
                node.value + std::cmp::max(left_sum, right_sum)
            }
            Some(path) => node.value + max_sum(Some(path.as_ref())),
        },
    }
}
