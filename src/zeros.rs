//This is from num-traits
// I am rewriting this for a learning purpose
pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self; 
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
    fn is_zero(&self) -> bool;
}