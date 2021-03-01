///Tree is a recursive data type with two forms: `Node` and `Leaf`.
///`Leaf` contains data.
///`Node` contains data and a list of `Tree`
#[derive(Debug)]
pub enum Tree<T> {
    Node(T, Vec<Tree>),
    Leaf(T)
}

impl<T> Tree<T> {

    /// Getter for data in tree
    pub fn data(&self) -> &T {
        match self {
            Tree::Leaf(d) => d,
            Tree::Node(d, _) => d
        }
    }
}

//impl fmt::Display for Tree {


#[cfg(test)]
mod tests {
    use super::*;
}
