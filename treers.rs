fn main() {
    let mut tree = Some(Box::new(Node {
        value: 1,
        left: Some(Box::new(Node { value: 2, left: Some(Box::new(Node { value: 3, left: None, right: None })), right: Some(Box::new(Node { value: 4, left: None, right: None })) })),
        right: Some(Box::new(Node { value: 5, left: Some(Box::new(Node { value: 6, left: None, right: None })), right: Some(Box::new(Node { value: 7, left: None, right: None })) }))
    }));
    println!("{:?}", tree);
    tree = invert_tree(tree);
    println!("{:?}", tree);
}

#[derive(Debug)]
struct Node {
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn invert_tree(n: Option<Box<Node>>) -> Option<Box<Node>> {
    match n {
        Some(node) => Some(Box::new(Node { value: node.value.clone(), left: invert_tree(node.right), right: invert_tree(node.left) })),
        None => None
    }
}