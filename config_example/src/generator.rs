use crate::{
    node::{formatter::NodeFormatter, Node},
    util::DocStr,
};

#[derive(Default)]
pub struct Example(Vec<Node>);

impl Example {
    pub fn add_comment(mut self, comment: impl Into<DocStr>) -> Self {
        self.0.push(Node::comment(comment));
        self
    }
}

pub trait NodeSchema {
    fn examples() -> impl IntoIterator<Item = Example>;
}

pub struct Generator;

impl Generator {
    pub fn generate<F, S>() -> impl IntoIterator<Item = String>
    where
        F: NodeFormatter,
        S: NodeSchema,
    {
        S::examples()
            .into_iter()
            .flat_map(|Example(nodes)| {
                nodes
                    .into_iter()
                    .map(|node| F::format_node(node))
                    .reduce(DocStr::merge)
            })
            .map(|doc_str| doc_str.to_string())
            .collect::<Vec<_>>()
    }
}
