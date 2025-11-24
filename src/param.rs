use std::fmt::Debug;

#[derive(PartialEq, Clone, Debug)]
pub struct Ref(String);

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Reference(Ref),
    RealNumber(f64),
    IntegerNumber(i64),
    NaturalNumber(u64),
    PlusOperator(Box<Expression>, Box<Expression>),
    MinusOperator(Box<Expression>, Box<Expression>),
    DivideOperator(Box<Expression>, Box<Expression>),
    MultiplyOperator(Box<Expression>, Box<Expression>),
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),
    CoTan(Box<Expression>),
    Arcsin(Box<Expression>),
    Arccos(Box<Expression>),
    Arctan(Box<Expression>)
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParamSet {
    pub parameters: Vec<Expression>,
}

impl ParamSet {
    pub fn add_parameter(&mut self, parameter: Expression) {
        self.parameters.push(parameter)
    }
}
