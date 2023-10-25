use std::cmp::PartialEq;
use std::fmt::Debug;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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

pub type Step<T> = fn(&mut Option<Rc<RefCell<T>>>, &mut Option<Vec<Arc<RwLock<T>>>>) -> Result<Option<StepMachineLabel>,StepMachineError>;

pub type ErrorHandler<T> = fn(StepLabel, StepMachineError, &mut Option<Rc<RefCell<T>>>, &mut Option<Vec<Arc<RwLock<T>>>>) -> StepMachineError;

pub struct StepMachine<T> 
where
    T: Debug
{
    sync_handler: Option<Rc<RefCell<T>>>,
    async_handlers: Option<Vec<Arc<RwLock<T>>>>,
    steps: HashMap<StepMachineLabel,Step<T>>,
    error_handler: Option<ErrorHandler<T>>
}

impl<T> StepMachine<T> 
where
    T: Debug
{
    pub fn new(sync_handler:Option<Rc<RefCell<T>>>, async_handlers:Option<Vec<Arc<RwLock<T>>>>, steps: Vec<(StepMachineLabel,Step<T>)>, error_handler: Option<ErrorHandler<T>>) -> Self {
        Self {
            sync_handler: sync_handler,
            async_handlers: async_handlers,
            steps: steps.into_iter().collect(),
            error_handler: error_handler
        }
    }

    pub fn run(&mut self, beginning:StepMachineLabel) -> Result<(),StepMachineError> {
        let mut last_step = beginning;
        if let Some(step) = self.steps.get(&last_step) {

            let mut result = step(&mut self.sync_handler, &mut self.async_handlers);

            while let Ok(res) = result {

                if res == Some(StepMachineLabel::Done) {
                    return Ok(());
                }

                if let Some(next_step) = res {

                    if let Some(step) = self.steps.get(&next_step) {
                        last_step = next_step;
                        result = step(&mut self.sync_handler, &mut self.async_handlers);
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
                        return Err(err_handler(last_step_label,error_code,&mut self.sync_handler,&mut self.async_handlers));
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