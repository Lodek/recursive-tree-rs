///Tree is a recursive data type with two forms: `Node` and `Leaf`.
///`Leaf` contains data.
///`Node` contains data and a list of `Tree`
#[derive(Debug)]
pub enum Tree<T> {
    Node(T, Vec<Tree<T>>),
    Leaf(T),
}

impl<T> Tree<T> {
    /// Getter for data in tree
    pub fn data(&self) -> &T {
        match self {
            Tree::Leaf(d) => d,
            Tree::Node(d, _) => d,
        }
    }

    /// Return vector of child trees if `self` is a Node,
    /// else returns the empty list.
    pub fn children(&self) -> Vec<&Tree<T>> {
        match self {
            Tree::Leaf(_) => Vec::new(),
            Tree::Node(_, trees) => trees.iter().collect()
        }
    }

    /// Depth first search fold implementation
    pub fn fold_dfs<F, U>(f: F, acc: U, tree: &Tree<T>) -> U
    where
        F: Fn(U, Tree<T>) -> U,
    {
        acc
    }

    /// Breadth first search fold implementation
    pub fn fold_bfs<F, U>(f: F, acc: U, tree: &Tree<T>) -> U
    where
        F: Fn(U, Tree<T>) -> U,
    {
        acc
    }
}

//impl fmt::Display for Tree {

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_factory() -> Tree<i32> {
        let c1 = Tree::Leaf(1);
        let c2 = Tree::Leaf(2);
        Tree::Node(3, vec![c1, c2])
    }

    #[test]
    fn children_return_childs_for_tree() {
        let tree = tree_factory();
        assert_eq!(tree.children().len(), 2);
    }
}
