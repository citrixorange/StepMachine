extern crate step_machine;

use std::cell::RefCell;
use std::rc::Rc;

use step_machine::{StepMachineError, StepLabel, StepError};
use crate::steps::labels::ExpressionSteps;
use crate::expression::expression::Expression;

#[repr(i32)]
pub enum ExpressionError {
    Underflow = 0,
    Overflow = 1,
    ZeroDivision = 2
}

impl From<ExpressionError> for StepMachineError {
    fn from(error: ExpressionError) -> StepMachineError {
        StepMachineError::StepError(error as StepError)
    }
}

pub fn error_handler(last_step: StepLabel, step_error: StepMachineError, handler: &mut Option<Rc<RefCell<Expression<u32>>>>) -> StepMachineError {
    
    let error: StepMachineError;

    if last_step == ExpressionSteps::Add.into() {
        error = StepMachineError::InternalError;
    } 
    
    else if last_step == ExpressionSteps::Subtract.into() {
        if step_error == ExpressionError::Underflow.into() {
            if let Some(hand) = &handler {
                let mut expression = hand.borrow_mut();
                expression.result = Some(0);
            }
            error = step_error;
        } else {
            error = StepMachineError::InternalError;
        }
    }

    else if last_step == ExpressionSteps::Multiply.into() {
        if step_error == ExpressionError::Overflow.into() {
            if let Some(hand) = &handler {
                let mut expression = hand.borrow_mut();
                expression.result = Some(u32::MAX);
            }
            error = step_error;
        } else {
            error = StepMachineError::InternalError;
        }
    }

    else if last_step == ExpressionSteps::Divide.into() {
        if step_error == ExpressionError::ZeroDivision.into() {
            if let Some(hand) = &handler {
                let mut expression = hand.borrow_mut();
                expression.result = None;
            }
            error = step_error;
        } else {
            error = StepMachineError::InternalError;
        }
    }

    else {
        error = StepMachineError::InternalError;
    }

    return error;

}