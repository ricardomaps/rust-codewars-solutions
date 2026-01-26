struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn tree_by_levels(root: &Node) -> Vec<u32> {
    // Debug implemented on Node to display its tree:
    //    println!("{root:?}");
    let mut res = Vec::new();

    fn bfs(level: Vec<&Node>, res: &mut Vec<u32>) {
        if level.len() == 0 {
            return;
        }
        let mut next_level = Vec::new();
        for node in level {
            if node.left.is_some() {
                next_level.push(&**node.left.as_ref().unwrap())
            }
            if node.right.is_some() {
                next_level.push(&**node.right.as_ref().unwrap())
            }
            res.push(node.value);
        }
        bfs(next_level, res)
    }

    bfs(vec![root], &mut res);

    res
}
