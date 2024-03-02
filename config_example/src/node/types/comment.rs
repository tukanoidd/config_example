use derive_more::From;

use crate::util::DocStr;

#[derive(Clone, From)]
pub struct CommentNode(pub DocStr);

impl CommentNode {
    pub fn new(str: impl Into<DocStr>) -> Self {
        Self(str.into())
    }
}

#[macro_export]
macro_rules! comment {
    ($line:expr) => {
        $crate::node::types::comment::CommentNode::new($line)
    };
    ($($line:expr),+) => {
        $crate::node::types::comment::CommentNode($crate::docstr_multi!([$($line),+]))
    };
}
