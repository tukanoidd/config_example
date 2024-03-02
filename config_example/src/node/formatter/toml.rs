use crate::{node::CommentNode, util::DocStr};

use super::NodeFormatter;

pub struct TomlNodeFormatter {}

impl NodeFormatter for TomlNodeFormatter {
    const NAMED_NODE_SEPARATOR: &'static str = " = ";

    fn format_comment(CommentNode(comment): CommentNode) -> DocStr {
        comment.map(|s| format!("# {s}"))
    }
}
