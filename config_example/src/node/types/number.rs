pub enum NumberNode {
    Integer(i64),
    Float(f64),
}

impl NumberNode {
    pub fn integer(int: impl IntoIntegerNode) -> Self {
        Self::Integer(int.into())
    }

    pub fn float(float: impl IntoFloatNode) -> Self {
        Self::Float(float.into())
    }
}

pub trait IntoIntegerNode: num::Integer + Into<i64> {}
pub trait IntoFloatNode: num::Float + Into<f64> {}

impl<I> IntoIntegerNode for I where I: num::Integer + Into<i64> {}
impl<F> IntoFloatNode for F where F: num::Float + Into<f64> {}

#[macro_export]
macro_rules! integer {
    ($int:expr) => {
        $crate::node::types::number::NumberNode::integer($int)
    };
}

#[macro_export]
macro_rules! float {
    ($float:expr) => {
        $crate::node::types::number::NumberNode::float($float)
    };
}
