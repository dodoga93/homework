use std::cmp::Ordering;

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>
}

impl<T> BST<T> where T:Ord {
    pub fn new() -> Self { BST{root: None} }

    pub fn insert(&mut self, elem: T) -> bool { self.root.insert(elem) }

    pub fn search(&self, elem: T) -> bool { self.root.search(elem) }
}

trait Set<T> {
    fn insert(&mut self, elem: T) -> bool;
    fn search(&self, elem: T) -> bool;
}

impl<T> Set<T> for Link<T> where T:Ord {
    fn insert(&mut self, elem: T) -> bool {
        match *self {
            None => {
                // place the element in this link
                *self = Some(Box::new(Node{elem: elem, left: None, right: None}));
                true
            }
            Some(ref mut node) => {
                match elem.cmp(&node.elem) {
                    Ordering::Equal => false,                    // return false if the is in this node
                    Ordering::Less => node.left.insert(elem),    // recurse to the left if the new value
                                                                 // is less than the node's value
                    Ordering::Greater => node.right.insert(elem) // recurse to the right if the new value
                                                                 // is greater than the node's value
                }
            }
        }
    }
    fn search(&self, elem: T) -> bool {
        match *self {
            None => false,
            Some(ref node) => 
                match elem.cmp(&node.elem) {
                    Ordering::Equal => true,
                    Ordering::Less => node.left.search(elem),
                    Ordering::Greater => node.right.search(elem)
                }
        }
    }
}

pub struct IntoIter<T>(Link<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(
            |boxed_node| {
                let node = *boxed_node;
                self.0 = node.right;
                node.elem
            }
        )
    }
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.root)
    }
}

pub struct Iter<'a, T: 'a>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(
            |node| {
                self.0 = node.right.as_ref().map(|boxed_node_ref| { &**boxed_node_ref } );
                &node.elem
            }
        )
    }
}

impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.root.as_ref().map(|boxed_node_ref| { &**boxed_node_ref } ))
    }
    
}

pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(
            |node| {
                self.0 = node.right.as_mut().take().map(|boxed_node_ref| { &mut **boxed_node_ref } );
                &mut node.elem
            }
        )
    }
}

impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.root.as_mut().take().map(|boxed_node_ref| { &mut **boxed_node_ref }))
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

    fn simple_bst() -> BST<i32> {
        let mut bst = BST::new();
        
        assert_eq!(bst.insert(1), true);
        assert_eq!(bst.insert(2), true);
        assert_eq!(bst.insert(3), true);

        bst
    }

    #[test]
    fn into_iter_for_test() {
        
        let elems = [1, 2, 3];
        
        let bst = simple_bst();

        let mut idx = 0;
        for elt in bst { // calls bst.into_iter()
            assert_eq!(elt, elems[idx]);
            idx += 1;
            println!("{}", elt);
        }

        assert_eq!(idx, 3);
    }
    
    #[test]
    fn iter_for_test() {
        let bst = simple_bst();

        print!("bst iter 1:");
        for elt in &bst {
            print!(" {}", elt);
        }
        print!("\n");

        print!("bst iter 2:");
        for elt in &bst {
            print!(" {}", elt);
        }
        print!("\n");
    }

    #[test]
    fn check_iter() {
        let bst = simple_bst();

        let mut iter = (& bst).into_iter();

        assert_eq!(iter.next(), Some(&1i32));
        assert_eq!(iter.next(), Some(&2i32));
        assert_eq!(iter.next(), Some(&3i32));
        
    }

    #[test]
    fn iter_mut_for_test() {
        let mut bst = simple_bst();

        print!("bst mut iter 1:");
        for elt in &mut bst {
            print!(" {}", elt);
        }
        print!("\n");

        print!("bst mut iter 2:");
        for elt in &mut bst {
            print!(" {}", elt);
        }
        print!("\n");
    }

        #[test]
    fn check_iter_mut() {
        let mut bst = simple_bst();

        let mut iter = (&mut bst).into_iter();

        assert_eq!(iter.next(), Some(&mut 1i32));
        assert_eq!(iter.next(), Some(&mut 2i32));
        assert_eq!(iter.next(), Some(&mut 3i32));
        
    }
}
