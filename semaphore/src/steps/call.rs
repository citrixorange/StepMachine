extern crate step_machine;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use step_machine::{StepMachineLabel, StepMachineError};
use crate::steps::labels::SemaphoreStates;

pub fn green(_sync_handler: &mut Option<Rc<RefCell<i32>>>, _async_handlers: &mut Option<Vec<Arc<RwLock<i32>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    println!("Semaphore State: GREEN");
    thread::sleep(Duration::from_secs(10));
    return Ok(Some(SemaphoreStates::Yellow.into()));
}

pub fn yellow(_sync_handler: &mut Option<Rc<RefCell<i32>>>, _async_handlers: &mut Option<Vec<Arc<RwLock<i32>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    println!("Semaphore State: YELLOW");
    thread::sleep(Duration::from_secs(3));
    return Ok(Some(SemaphoreStates::Red.into()));
}

pub fn red(_sync_handler: &mut Option<Rc<RefCell<i32>>>, _async_handlers: &mut Option<Vec<Arc<RwLock<i32>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    println!("Semaphore State: RED");
    thread::sleep(Duration::from_secs(10));
    return Ok(Some(SemaphoreStates::Green.into()));
}