use macros::node;

use crate::{
    comment,
    node::{formatter::NodeFormatter, types::number::NumberNode, Node},
    util::DocStr,
};

#[derive(Default)]
pub struct Example(Vec<Node>);

impl Example {
    pub fn from_nodes(nodes: impl IntoIterator<Item = Node>) -> Self {
        Self::from_nodes_iter(nodes.into_iter())
    }

    pub fn from_nodes_iter(nodes: impl Iterator<Item = Node>) -> Self {
        Self(nodes.collect())
    }

    pub fn add_comment(mut self, comment: impl Into<DocStr>) -> Self {
        self.0.push(node!(comment!(comment)));
        self
    }

    pub fn add_number(mut self, number: NumberNode) -> Self {
        self.0.push(node!(number));
        self
    }
}

pub trait NodeSchema {
    fn examples() -> impl IntoIterator<Item = Example>;
}

pub struct Generator;

impl Generator {
    pub fn generate<F, S>() -> Vec<String>
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

#[cfg(test)]
mod test {
    use macros::node;

    use crate::{
        comment, float, integer,
        node::formatter::{toml::TomlNodeFormatter, yaml::YamlFormatter},
    };

    use super::{Example, Generator, NodeSchema};

    struct TestSchema {}

    impl NodeSchema for TestSchema {
        fn examples() -> impl IntoIterator<Item = Example> {
            [Example::from_nodes([
                node!(comment!("This", "is", "a", "multiline", "comment")),
                node!(integer!(5).with_name("test_int")),
                node!(float!(4.6).with_name("test_float")),
            ])]
        }
    }

    #[test]
    fn print_out_test_schema() {
        let generated_toml_str = Generator::generate::<TomlNodeFormatter, TestSchema>().join("\n");
        let generated_yaml_str = Generator::generate::<YamlFormatter, TestSchema>().join("\n");

        println!("Generated TOML example:\n{generated_toml_str}");
        println!();
        println!("Generated YAML example:\n{generated_yaml_str}");
    }
}
