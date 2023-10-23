use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::rc::Rc;

pub type StepLabel = i32;

#[repr(i32)]
pub enum StepMachineError {
    InternalError = 0,
    InexistentStep = 1,
    StepError(i32)
}

pub type Step<T> = fn(Option<Rc<T>>, Option<Vec<Arc<RwLock<T>>>>) -> Result<Option<StepLabel>,StepMachineError>;

pub type ErrorHandler<T> = fn(StepLabel, StepMachineError, Option<Rc<T>>, Option<Vec<Arc<RwLock<T>>>>) -> StepMachineError;

pub struct StepMachine<T> {
    sync_handler: Option<Rc<T>>,
    async_handlers: Option<Vec<Arc<RwLock<T>>>>,
    steps: HashMap<StepLabel,Step<T>>,
    error_handler: Option<ErrorHandler<T>>
}

impl<T> StepMachine<T> {
    pub fn new(sync_handler:Option<Rc<T>>, async_handlers:Option<Vec<Arc<RwLock<T>>>>, steps: Vec<(StepLabel,Step<T>)>, error_handler: Option<ErrorHandler<T>>) -> Self {
        Self {
            sync_handler: sync_handler,
            async_handlers: async_handlers,
            steps: steps.into_iter().collect(),
            error_handler: error_handler
        }
    }

    pub fn run(&mut self, beginning:StepLabel) -> Result<(),StepMachineError> {
        let mut last_step = beginning;
        if let Some(step) = self.steps.get(&last_step) {

            let mut sync_handler = None;
            let mut async_handlers = None;

            if let Some(handler) = &self.sync_handler {
                sync_handler = Some(Rc::clone(&handler));
            }

            if let Some(handler) = &self.async_handlers {
                async_handlers = Some(handler.iter().cloned().collect());
            }

            let mut result = step(sync_handler, async_handlers);

            while let Ok(res) = result {
                if let Some(next_step) = res {

                    if let Some(step) = self.steps.get(&next_step) {

                        last_step = next_step;

                        let mut sync_handler = None;
                        let mut async_handlers = None;
            
                        if let Some(handler) = &self.sync_handler {
                            sync_handler = Some(Rc::clone(&handler));
                        }
            
                        if let Some(handler) = &self.async_handlers {
                            async_handlers = Some(handler.iter().cloned().collect());
                        }

                        result = step(sync_handler, async_handlers);
                    } else {
                        return Err(StepMachineError::InexistentStep);
                    }
                } else {
                    return Ok(());
                }
            }

            if let Err(error_code) = result {
                if let Some(err_handler) = self.error_handler {

                    let mut sync_handler = None;
                    let mut async_handlers = None;
        
                    if let Some(handler) = &self.sync_handler {
                        sync_handler = Some(Rc::clone(&handler));
                    }
        
                    if let Some(handler) = &self.async_handlers {
                        async_handlers = Some(handler.iter().cloned().collect());
                    }

                    return Err(err_handler(last_step,error_code,sync_handler,async_handlers));
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