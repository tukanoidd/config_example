pub mod comment;
pub mod number;

use derive_more::From;

use self::{comment::CommentNode, number::NumberNode};

#[derive(From)]
pub enum NodeType {
    Comment(CommentNode),
    Empty,
    EmptyMultiline(usize),
    Number(NumberNode),
}

#[macro_export]
macro_rules! empty {
    () => {
        $crate::node::types::NodeType::Empty
    };
    ($amount:expr) => {
        $crate::node::types::NodeType::EmptyMultiline($amount)
    };
}
