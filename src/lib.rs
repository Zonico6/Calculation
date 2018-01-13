pub mod math;
mod bricks;
pub mod builder;

use std::collections::HashMap;
use std::rc::Rc;

/// Type of all numbers inside an equation
pub type Num = f32;
/// Type for all values, labelling operator priorities
pub type Prior = f32;

type UnaryOperation = Rc<Fn(Num) -> Num>;
type BinaryOperation = Rc<Fn(Num, Num) -> Num>;

type UnaryOperationMap = HashMap<String, (UnaryOperation, Prior)>;
type BinaryOperationMap = HashMap<String, (BinaryOperation, Prior)>;
type AppendedOperationMap = HashMap<String, UnaryOperation>;

pub struct Environment {
    pub un_operations: UnaryOperationMap,
    pub bin_operations: BinaryOperationMap,
    pub app_operations: AppendedOperationMap,
}