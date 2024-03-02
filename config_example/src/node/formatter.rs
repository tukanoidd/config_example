use crate::{docstr_empty, util::DocStr};

use super::{types::number::NumberNode, CommentNode, Comments, Node, NodeType};

#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "yaml")]
pub mod yaml;

pub trait NodeFormatter {
    fn format_node(
        Node {
            tabs,
            ty,
            comments: Comments { top, right },
        }: Node,
    ) -> DocStr {
        let res = match ty {
            NodeType::Comment(comment) => Self::format_comment(comment.clone()),
            NodeType::Empty => Self::format_empty(),
            NodeType::EmptyMultiline(amount) => Self::format_empty_multiline(amount),
            NodeType::Number(num) => Self::format_number(num),
        };

        match (top, right) {
            (None, None) => res,
            (None, Some(CommentNode(right_str))) => res.attach_right(right_str),
            (Some(CommentNode(top_str)), None) => top_str.merge(res),
            (Some(CommentNode(top_str)), Some(CommentNode(right_str))) => {
                top_str.merge(res).attach_right(right_str)
            }
        }
        .tabbed(tabs)
    }

    fn format_comment(comment: CommentNode) -> DocStr;

    fn format_empty() -> DocStr {
        docstr_empty!()
    }

    fn format_empty_multiline(amount: usize) -> DocStr {
        docstr_empty!(amount)
    }

    fn format_number(number: NumberNode) -> DocStr {
        DocStr::line(match number {
            NumberNode::Integer(int) => int.to_string(),
            NumberNode::Float(float) => float.to_string(),
        })
    }
}
