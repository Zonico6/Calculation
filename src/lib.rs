pub mod math;
mod bricks;
pub mod builder;

use std::collections::HashMap;

/// Type of all numbers inside an equation
pub type Num = f32;
/// Type for all values, labelling operator priorities
pub type Prior = f32;

type UnaryOperationMap = HashMap<String, (Box<Fn(Num) -> Num>, Prior)>;
type BinaryOperationMap = HashMap<String, (Box<Fn(Num, Num) -> Num>, Prior)>;
type AppendedOperationMap = HashMap<String, Box<Fn(Num) -> Num>>;

pub struct Environment {
    pub un_operations: UnaryOperationMap,
    pub bin_operations: BinaryOperationMap,
    pub app_operations: AppendedOperationMap,
}