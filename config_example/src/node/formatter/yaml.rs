use crate::{node::CommentNode, util::DocStr};

use super::NodeFormatter;

pub struct YamlFormatter;

impl NodeFormatter for YamlFormatter {
    const NAMED_NODE_SEPARATOR: &'static str = ": ";

    fn format_comment(CommentNode(comment): CommentNode) -> DocStr {
        comment.map(|s| format!("# {s}"))
    }
}
