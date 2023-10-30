extern crate sync_step_machine;

use std::cell::RefCell;
use std::rc::Rc;

use sync_step_machine::{StepMachineLabel, Step, StepMachine};
use crate::steps::labels::ExpressionSteps;
use crate::steps::errors::{ExpressionError, error_handler};
use crate::steps::call::{add, sub, mult, div};
use crate::expression::expression::Expression;

mod steps;
mod expression;

fn main() {

    let steps:Vec<(StepMachineLabel,Step<Expression<u32>>)>  = vec![
        (ExpressionSteps::Add.into(), |x| add(x)),
        (ExpressionSteps::Subtract.into(), |x| sub(x)),
        (ExpressionSteps::Multiply.into(), |x| mult(x)),
        (ExpressionSteps::Divide.into(), |x| div(x))
    ];

    let expression: Expression<u32> = Expression::new(Some(5),Some(4),Some(2),Some(2));
    let ptr_expression = Rc::new(RefCell::new(expression));
    let mut step_machine = StepMachine::<Expression<u32>>::new(Some(Rc::clone(&ptr_expression)), steps, Some(|x,y,z| error_handler(x,y,z)));
    let result = step_machine.run(ExpressionSteps::Add.into());
    assert!(result == Ok(()));
    let value = ptr_expression.borrow();
    assert!(value.result == Some(1));

    let steps:Vec<(StepMachineLabel,Step<Expression<u32>>)>  = vec![
        (ExpressionSteps::Add.into(), |x| add(x)),
        (ExpressionSteps::Subtract.into(), |x| sub(x)),
        (ExpressionSteps::Multiply.into(), |x| mult(x)),
        (ExpressionSteps::Divide.into(), |x| div(x))
    ];

    let expression: Expression<u32> = Expression::new(Some(5),Some(6),Some(2),Some(2));
    let ptr_expression = Rc::new(RefCell::new(expression));
    let mut step_machine = StepMachine::<Expression<u32>>::new(Some(Rc::clone(&ptr_expression)), steps, Some(|x,y,z| error_handler(x,y,z)));
    let result = step_machine.run(ExpressionSteps::Add.into());
    assert!(result == Err(ExpressionError::Underflow.into()));
    let value = ptr_expression.borrow();
    assert!(value.result == Some(0));

    let steps:Vec<(StepMachineLabel,Step<Expression<u32>>)>  = vec![
        (ExpressionSteps::Add.into(), |x| add(x)),
        (ExpressionSteps::Subtract.into(), |x| sub(x)),
        (ExpressionSteps::Multiply.into(), |x| mult(x)),
        (ExpressionSteps::Divide.into(), |x| div(x))
    ];

    let expression: Expression<u32> = Expression::new(Some(4000000000),Some(0),Some(500000000),Some(1));
    let ptr_expression = Rc::new(RefCell::new(expression));
    let mut step_machine = StepMachine::<Expression<u32>>::new(Some(Rc::clone(&ptr_expression)), steps, Some(|x,y,z| error_handler(x,y,z)));
    let result = step_machine.run(ExpressionSteps::Add.into());
    assert!(result == Err(ExpressionError::Overflow.into()));
    let value = ptr_expression.borrow();
    assert!(value.result == Some(std::u32::MAX));

    let steps:Vec<(StepMachineLabel,Step<Expression<u32>>)>  = vec![
        (ExpressionSteps::Add.into(), |x| add(x)),
        (ExpressionSteps::Subtract.into(), |x| sub(x)),
        (ExpressionSteps::Multiply.into(), |x| mult(x)),
        (ExpressionSteps::Divide.into(), |x| div(x))
    ];

    let expression: Expression<u32> = Expression::new(Some(4),Some(0),Some(5),Some(0));
    let ptr_expression = Rc::new(RefCell::new(expression));
    let mut step_machine = StepMachine::<Expression<u32>>::new(Some(Rc::clone(&ptr_expression)), steps, Some(|x,y,z| error_handler(x,y,z)));
    let result = step_machine.run(ExpressionSteps::Add.into());
    assert!(result == Err(ExpressionError::ZeroDivision.into()));
    let value = ptr_expression.borrow();
    assert!(value.result == None);


}
