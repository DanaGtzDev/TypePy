use std::rc::Rc;
use std::cell::RefCell;

// Define a struct for the tree node
#[derive(Debug)]
pub struct ASTNode<T> {
    value: T,
    children: Vec<Rc<RefCell<ASTNode<T>>>>,
}

// Implement methods for the tree node
impl<T> ASTNode<T> {
    // Create a new tree node
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ASTNode {
            value,
            children: Vec::new(),
        }))
    }

    // Add a child to the tree node
    fn add_child(parent: Rc<RefCell<ASTNode<T>>>, child: Rc<RefCell<ASTNode<T>>>) {
        parent.borrow_mut().children.push(child);
    }
}

// Define a struct for the tree
#[derive(Debug)]
pub struct AST<T> {
    root: Option<Rc<RefCell<ASTNode<T>>>>,
}

// Implement methods for the tree
impl<T> AST<T> {
    // Create a new, empty tree
    pub fn new() -> Self {
        AST { root: None }
    }

    // Set the root of the tree
    pub fn set_root(&mut self, value: T) -> Rc<RefCell<ASTNode<T>>> {
        let root = ASTNode::new(value);
        self.root = Some(root.clone());
        root
    }
}
