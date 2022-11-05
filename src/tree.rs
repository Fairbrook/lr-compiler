use crate::token::Token;

pub enum TreeItem {
    Leaf(Token),
    Tree()
}

pub struct Tree{
    root: String,
}
