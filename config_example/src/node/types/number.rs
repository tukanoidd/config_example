pub struct NumberNode {
    pub name: String,
    pub ty: NumberType,
}

impl NumberNode {
    pub fn new(name: impl Into<String>, ty: NumberType) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }

    pub fn integer(name: impl Into<String>, int: impl IntoIntegerType) -> Self {
        Self::new(name, NumberType::integer(int))
    }

    pub fn float(name: impl Into<String>, float: impl IntoFloatType) -> Self {
        Self::new(name, NumberType::float(float))
    }
}

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
    ($name:expr, $int:expr) => {
        $crate::node::types::number::NumberNode::integer($name, $int)
    };
}

#[macro_export]
macro_rules! float {
    ($name:expr, $float:expr) => {
        $crate::node::types::number::NumberNode::float($name, $float)
    };
}
