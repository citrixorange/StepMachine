use std::cmp::PartialEq;
use std::fmt::Debug;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type StepLabel = i32;
pub type StepError = i32;

#[derive(Eq, PartialEq, Hash)]
pub enum StepMachineLabel {
    Done,
    StepLabel(StepLabel)
}

#[derive(Eq, PartialEq)]
pub enum StepMachineError {
    InternalError,
    InexistentStep,
    StepError(StepError)
}

pub type Step<T> = fn(&mut Option<Rc<RefCell<T>>>) -> Result<Option<StepMachineLabel>,StepMachineError>;

pub type ErrorHandler<T> = fn(StepLabel, StepMachineError, &mut Option<Rc<RefCell<T>>>) -> StepMachineError;

pub struct StepMachine<T> 
where
    T: Debug
{
    handler: Option<Rc<RefCell<T>>>,
    steps: HashMap<StepMachineLabel,Step<T>>,
    error_handler: Option<ErrorHandler<T>>
}

impl<T> StepMachine<T> 
where
    T: Debug
{
    pub fn new(handler:Option<Rc<RefCell<T>>>, steps: Vec<(StepMachineLabel,Step<T>)>, error_handler: Option<ErrorHandler<T>>) -> Self {
        Self {
            handler: handler,
            steps: steps.into_iter().collect(),
            error_handler: error_handler
        }
    }

    pub fn run(&mut self, beginning:StepMachineLabel) -> Result<(),StepMachineError> {
        let mut last_step = beginning;
        if let Some(step) = self.steps.get(&last_step) {

            let mut result = step(&mut self.handler);

            while let Ok(res) = result {

                if res == Some(StepMachineLabel::Done) {
                    return Ok(());
                }

                if let Some(next_step) = res {

                    if let Some(step) = self.steps.get(&next_step) {
                        last_step = next_step;
                        result = step(&mut self.handler);
                    } else {
                        return Err(StepMachineError::InexistentStep);
                    }
                } else {
                    return Ok(());
                }
            }

            if let Err(error_code) = result {
                if let Some(err_handler) = self.error_handler {

                    if let StepMachineLabel::StepLabel(last_step_label) = last_step {
                        return Err(err_handler(last_step_label,error_code,&mut self.handler));
                    } else {
                        return Err(StepMachineError::InternalError);
                    }

                }
                return Err(error_code);
            } else {
                return Err(StepMachineError::InternalError);
            }

        } else {
            return Err(StepMachineError::InexistentStep);
        }
    }
}