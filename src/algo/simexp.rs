use std::string;

#[derive(Debug, Clone)]
pub struct SimExp {
    exp: String,
}

/* Helper Functions */
pub fn is_val() -> bool {true}

pub fn is_var() -> bool {true}

pub fn is_whitespace() -> bool {true}

pub fn is_bracket() -> bool {true}

pub fn is_operator() -> bool {true}

pub fn evaluate(mut expression: String) {
    let expression_size = expression.len();
    let iter = expression.split_whitespace().collect::<std::vec::Vec<&str>>;
    for i in 1..expression_size {
        let (before, sliced) = expression.split_at_mut(i);
        println!("{}", before);
        assert_eq!(before, "h");
    }
}
impl SimExp {
    
    pub fn new(expression: &str) -> Self {
        return Self { exp : String::from(expression)};
    }

    pub fn wrap(&self) -> String {
        return String::from("(".to_owned() + &self.exp +")");
    }

    pub fn evaluate() -> String {
        String::from("valuHelloe")
    }

}

impl std::ops::Add for SimExp {
    type Output = SimExp;

    fn add(self: Self, rhs: Self) -> Self::Output{
        return Self {exp: self.wrap() + &rhs.wrap()};
    }
}

impl std::fmt::Display for SimExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sym({})", self.exp)
    }
}