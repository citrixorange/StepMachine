extern crate step_machine;

use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use step_machine::{StepLabel, StepMachineError};
use crate::steps::labels::SemaphoreStates;

pub fn green(_sync_handler: Option<Rc<i32>>, _async_handlers: Option<Vec<Arc<RwLock<i32>>>>) -> Result<Option<StepLabel>,StepMachineError> {
    println!("Semaphore State: GREEN");
    thread::sleep(Duration::from_secs(10));
    return Ok(Some(SemaphoreStates::Yellow.into()));
}

pub fn yellow(_sync_handler: Option<Rc<i32>>, _async_handlers: Option<Vec<Arc<RwLock<i32>>>>) -> Result<Option<StepLabel>,StepMachineError> {
    println!("Semaphore State: YELLOW");
    thread::sleep(Duration::from_secs(3));
    return Ok(Some(SemaphoreStates::Red.into()));
}

pub fn red(_sync_handler: Option<Rc<i32>>, _async_handlers: Option<Vec<Arc<RwLock<i32>>>>) -> Result<Option<StepLabel>,StepMachineError> {
    println!("Semaphore State: RED");
    thread::sleep(Duration::from_secs(10));
    return Ok(Some(SemaphoreStates::Green.into()));
}