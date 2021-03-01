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

    /// Depth first search fold implementation
    pub fn fold_dfs<F, U>(f: F, acc: U, tree: &Tree<T>) -> U
    where
        F: Fn(U, Tree<T>) -> U,
    {
        acc
    }

    /// Breadth first search fold implementation
    pub fn fold_dfs<F, U>(f: F, acc: U, tree: &Tree<T>) -> U
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
}
