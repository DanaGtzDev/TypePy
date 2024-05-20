use crate::lib::cfg;
use crate::lib::tokenizer::Token;
use super::ast::AST;


pub fn tree_generator(tokens: Vec<Token>) -> AST<String>{
    /*
        // Create a new tree
        let mut tree = AST::new();

        // Set the root of the tree
        let root = tree.set_root("root");

        // Add children to the root
        let child1 = ASTNode::new("child1");
        let child2 = ASTNode::new("child2");
        ASTNode::add_child(root.clone(), child1.clone());
        ASTNode::add_child(root.clone(), child2.clone());

        // Add children to one of the root's children
        let grandchild1 = ASTNode::new("grandchild1");
        let grandchild2 = ASTNode::new("grandchild2");
        ASTNode::add_child(child1.clone(), grandchild1);
        ASTNode::add_child(child1, grandchild2);

        // Print the tree structure
        println!("{:#?}", tree);
     */

     let mut tree = AST::new();
     return tree;
    
    

}