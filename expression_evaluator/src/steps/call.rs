extern crate step_machine;

use std::sync::{Arc, RwLock};
use std::cell::RefCell;
use std::rc::Rc;

use step_machine::{StepMachineLabel, StepMachineError};
use crate::steps::labels::ExpressionSteps;
use crate::steps::errors::ExpressionError;
use crate::expression::expression::Expression;

pub fn add(sync_handler: &mut Option<Rc<RefCell<Expression<u32>>>>, _async_handlers: &mut Option<Vec<Arc<RwLock<Expression<u32>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    if let Some(handler) = &sync_handler {
        let mut expression = handler.borrow_mut();
        if let Some(add) = expression.add {
            expression.result = Some(add);
        }
        return Ok(Some(ExpressionSteps::Subtract.into()));
    } else {
        return Ok(Some(ExpressionSteps::Subtract.into()));
    }
}

pub fn sub(sync_handler: &mut Option<Rc<RefCell<Expression<u32>>>>, _async_handlers: &mut Option<Vec<Arc<RwLock<Expression<u32>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    if let Some(handler) = &sync_handler {
        let mut expression = handler.borrow_mut();
        if let Some(sub) = expression.subtract {
            if let Some(result) = expression.result {
                if result > sub {
                    expression.result = Some(result-sub);
                } else {
                    return Err(ExpressionError::Underflow.into());
                }
            } else {
                return Err(ExpressionError::Underflow.into());
            }
        }
        return Ok(Some(ExpressionSteps::Multiply.into()));
    } else {
        return Ok(Some(ExpressionSteps::Multiply.into()));
    }
}

pub fn mult(sync_handler: &mut Option<Rc<RefCell<Expression<u32>>>>, _async_handlers: &mut Option<Vec<Arc<RwLock<Expression<u32>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    if let Some(handler) = &sync_handler {
        let mut expression = handler.borrow_mut();
        if let Some(mult) = expression.multiply {
            if let Some(result) = expression.result {
                match result.checked_mul(mult) {
                    Some(res) => {
                        expression.result = Some(res);
                    }
                    None => {
                        return Err(ExpressionError::Overflow.into());
                    }
                }
            }
        }
        return Ok(Some(ExpressionSteps::Divide.into()));
    } else {
        return Ok(Some(ExpressionSteps::Divide.into()));
    }
}

pub fn div(sync_handler: &mut Option<Rc<RefCell<Expression<u32>>>>, _async_handlers: &mut Option<Vec<Arc<RwLock<Expression<u32>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    if let Some(handler) = &sync_handler {
        let mut expression = handler.borrow_mut();
        if let Some(divide) = expression.divide {
            if let Some(result) = expression.result {
                if divide == 0 {
                    return Err(ExpressionError::ZeroDivision.into());
                }
                expression.result = Some(result/divide);
            }
        }
        return Ok(Some(StepMachineLabel::Done));
    } else {
        return Ok(Some(StepMachineLabel::Done));
    }
}