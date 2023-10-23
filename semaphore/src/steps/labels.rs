extern crate step_machine;

use step_machine::{StepLabel};

#[repr(i32)]
pub enum SemaphoreStates {
    Green = 0,
    Yellow = 1,
    Red = 2
}

impl From<SemaphoreStates> for StepLabel {
    fn from(state: SemaphoreStates) -> StepLabel {
        state as StepLabel
    }
}