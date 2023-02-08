use std::vec::Vec;

pub trait TnesorForm {}

#[derive(Debug, Clone)]
pub struct Tensor {
    pub dim: i32,
    pub index: Option<String>,
    pub values: Vec<Tensor>
}

pub struct Tangent {
    dim: i32,
    index: String,
    val: [Scalar],
}

pub struct OneForm {
    dim: i32,
    index: String,
}
pub enum Scalar {
    String(String),
    Float(f64),
}

impl std::ops::Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Scalar {
        return self;
    }
}

pub trait Transpose {
    fn transpose(&self) -> Self;
}

pub trait ContractIndex {
    fn contract(&self) -> Self;
}

impl std::ops::Add for OneForm {
    type Output = OneForm;

    fn add(self, rhs: OneForm) ->  OneForm {
        return self;
    }

}