use algo::tensors::{Tensor, OneForm};
use algo::simexp::*;
use algo::complexnumbers::*;

pub mod algo;

fn main() {
    let a = Complex::new(0.0, 0.0);
    let b: SimExp = SimExp::new("Hello");
    println!("{} and {}", a, b.wrap());
    algo::simexp::evaluate(String::from("hello"));
}