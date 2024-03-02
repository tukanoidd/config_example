pub mod formatter;
pub mod types;

use macros::node;

use crate::{comment, empty, float, integer, util::DocStr};

use self::types::{
    comment::CommentNode,
    number::{IntoFloatNode, IntoIntegerNode},
    NodeType,
};

pub struct Node {
    ty: NodeType,
    tabs: usize,
    comments: Comments,
}

impl Node {
    pub fn new(ty: impl Into<NodeType>) -> Self {
        Self {
            ty: ty.into(),
            tabs: 0,
            comments: Default::default(),
        }
    }

    pub fn comment(comment: impl Into<DocStr>) -> Self {
        Self::new(comment!(comment))
    }

    pub fn integer(int: impl IntoIntegerNode) -> Self {
        Self::new(integer!(int))
    }

    pub fn float(float: impl IntoFloatNode) -> Self {
        Self::new(float!(float))
    }

    pub fn empty() -> Self {
        Self::new(empty!())
    }

    pub fn empty_multiline(amount: usize) -> Self {
        Self::new(empty!(amount))
    }

    pub fn with_tabs(mut self, tabs: usize) -> Self {
        self.tabs = tabs;
        self
    }

    pub fn with_top_comment(mut self, comment: impl Into<CommentNode>) -> Self {
        self.comments.top = Some(comment.into());
        self
    }

    pub fn with_right_comment(mut self, comment: impl Into<CommentNode>) -> Self {
        self.comments.right = Some(comment.into());
        self
    }
}

#[derive(Default)]
pub struct Comments {
    top: Option<CommentNode>,
    right: Option<CommentNode>,
}

fn test() {
    let node = node!(
        integer!(2),
        tabs = 2,
        comments = {
            top: "hallo",
            right: ["hallo", "bye"]
        }
    );
}
