use algo::complexnumbers::*;
use algo::symexp::*;
use algo::tensors::{OneForm, Tensor};

pub mod algo;

fn main() {
    let a = Complex::new(0.0, 0.0);
    let b: SymExp = SymExp::new("Hello");
    println!("{} and {}", a, b.wrap());
}
