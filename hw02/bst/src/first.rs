use std::cmp::Ordering;

#[derive(Debug)]
pub struct BST {
    root: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link
}

impl BST {
    pub fn new() -> Self { BST{root: Link::Empty} }

    pub fn insert(&mut self, elem: i32) -> bool { self.root.insert(elem) }

    pub fn search(&self, elem: i32) -> bool { self.root.search(elem) }
}

impl Link {
    fn insert(&mut self, elem: i32) -> bool {
        match *self {
            Link::Empty => {
                // place the element in this link
                *self = Link::More(Box::new(Node{elem: elem, left: Link::Empty, right: Link::Empty}));
                true
            }
            Link::More(ref mut node) => {
                match node.elem.cmp(&elem) {
                    Ordering::Equal => false,                    // return false if the is in this node
                    Ordering::Less => node.left.insert(elem),    // recurse to the left if the new value
                                                                 // is less than the node's value
                    Ordering::Greater => node.right.insert(elem) // recurse to the right if the new value
                                                                 // is greater than the node's value
                }
            }
        }
    }
    fn search(&self, elem: i32) -> bool {
        match *self {
            Link::Empty => false,
            Link::More(ref node) => 
                match node.elem.cmp(&elem) {
                    Ordering::Equal => true,
                    Ordering::Less => node.left.search(elem),
                    Ordering::Greater => node.right.search(elem)
                }
        }
    }
}


#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_insert_search() {
        let mut bin_tree = BST::new();

        assert_eq!(bin_tree.search(5), false);

        assert_eq!(bin_tree.insert(5), true);
        assert_eq!(bin_tree.insert(6), true);
        assert_eq!(bin_tree.insert(2), true);
        assert_eq!(bin_tree.insert(3), true);

        assert_eq!(bin_tree.insert(3), false);

        assert_eq!(bin_tree.search(6), true);
        assert_eq!(bin_tree.search(7), false);
        assert_eq!(bin_tree.search(3), true);
    }
}
