use derive_more::Display;

pub struct NumberNode {
    pub ty: NumberType,
    pub name: Option<String>,
}

impl NumberNode {
    pub fn new(ty: NumberType) -> Self {
        Self { ty, name: None }
    }

    pub fn integer(int: impl IntoIntegerType) -> Self {
        Self::new(NumberType::integer(int))
    }

    pub fn float(float: impl IntoFloatType) -> Self {
        Self::new(NumberType::float(float))
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

#[derive(Display)]
pub enum NumberType {
    Integer(i64),
    Float(f64),
}

impl NumberType {
    pub fn integer(int: impl IntoIntegerType) -> Self {
        Self::Integer(int.into())
    }

    pub fn float(float: impl IntoFloatType) -> Self {
        Self::Float(float.into())
    }
}

pub trait IntoIntegerType: num::Integer + Into<i64> {}
pub trait IntoFloatType: num::Float + Into<f64> {}

impl<I> IntoIntegerType for I where I: num::Integer + Into<i64> {}
impl<F> IntoFloatType for F where F: num::Float + Into<f64> {}

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
