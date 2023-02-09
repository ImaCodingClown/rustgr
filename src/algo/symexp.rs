use std::{collections::HashMap, string};

#[derive(Debug, Clone)]
pub struct SymExp {
    pub exp: String,
}

#[derive(Debug, Clone)]
pub struct SymExpConstraints {
    pub var: HashMap<String, f64>,
}

/* Helper Functions */
pub fn is_val() -> bool {
    true
}

pub fn is_var() -> bool {
    true
}

pub fn is_bracket() -> bool {
    true
}

pub fn is_operator() -> bool {
    true
}

// pub fn evaluate(mut expression: String) {
//     let expression_size = expression.len();
//     let iter: std::vec::Vec<&str> = expression.split_whitespace().collect();
//     for i in 1..expression_size {
//         let (before, sliced) = expression.split_at_mut(i);
//         println!("{}", before);
//         assert_eq!(before, "h");
//     }
// }
impl SymExp {
    pub fn new(expression: &str) -> Self {
        return Self {
            exp: String::from(expression),
        };
    }

    pub fn wrap(&self) -> String {
        return String::from("(".to_owned() + &self.exp + ")");
    }

    pub fn evaluate() -> String {
        String::from("valuHelloe")
    }
}

impl std::ops::Add for SymExp {
    type Output = SymExp;

    fn add(self: Self, rhs: Self) -> Self::Output {
        return Self {
            exp: self.wrap() + &rhs.wrap(),
        };
    }
}

impl std::fmt::Display for SymExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sym({})", self.exp)
    }
}
