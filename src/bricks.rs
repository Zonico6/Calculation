use self::super::{Prior, Num, BinaryOperation, UnaryOperation};

/// A brick with a unary operand
pub struct UnaryBrick {
    priority: Prior,
    operation: UnaryOperation,
    prev: Option<Box<Brick>>,
}
impl UnaryBrick {
    pub fn new(last_brick: Option<Box<Brick>>, operation: UnaryOperation, priority: Prior) -> UnaryBrick {
        UnaryBrick {prev: last_brick, operation, priority}
    }
}
impl Brick for UnaryBrick {
    fn resolve(self: Box<Self>, v: Num) -> (Num, Option<Box<Brick>>) {
        let value = (self.operation)(v);
        /*if self.prev.is_some() {
            self.prev.unwrap().resolve(value)
        } else {
            (value, None)
        }*/
        match self.prev {
            Some(prev_brick) => prev_brick.resolve(value),
            None => (value, None)
        }
    }
    fn get_priority(&self) -> Prior {
        self.priority
    }
}
/// A brick with a binary operand
pub struct BinaryBrick {
    priority: Prior,
    operation: BinaryOperation,
    value: Num,
    prev: Option<Box<Brick>>
}
impl BinaryBrick {
    pub fn new(last_brick: Option<Box<Brick>>, value: Num, operation: BinaryOperation, priority: Prior) -> BinaryBrick {
        let mut brick = BinaryBrick {prev: last_brick, value, operation, priority};
        if let Some(last_brick) = brick.prev {
            if brick.priority > last_brick.get_priority() {
                let res = last_brick.resolve(brick.value);
                brick.value = res.0;
                brick.prev = res.1;
            } else {
                brick.prev = Some(last_brick);
            }
        }
        brick
    }
}
impl Brick for BinaryBrick {
    fn resolve(self: Box<Self>, v: Num) -> (Num, Option<Box<Brick>>) {
        let value = (self.operation)(self.value, v);
        match self.prev {
            Some(prev_brick) => prev_brick.resolve(value),
            None => (value, None)
        }
    }
    fn get_priority(&self) -> Prior {
        self.priority
    }
}
/// The Brick that emulates a Bracket
pub struct Bracket {
    prev: Option<Box<Brick>>
}
impl Bracket {
    pub fn new(last_brick: Option<Box<Brick>>) -> Bracket {
        Bracket {prev: last_brick}
    }
}
impl Brick for Bracket {
    fn resolve(self: Box<Self>, v: Num) -> (Num, Option<Box<Brick>>) {
        (v, self.prev)
    }
}
pub trait Brick {
    /// Takes the number ahead, returns the result and, in case of a bracket its back brick
    fn resolve(self: Box<Self>, v: Num) -> (Num, Option<Box<Brick>>);
    fn get_priority(&self) -> Prior {
        1f32
    }
}
