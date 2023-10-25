extern crate step_machine;

use step_machine::{StepLabel, StepMachineLabel};

#[repr(i32)]
pub enum SemaphoreStates {
    Green = 0,
    Yellow = 1,
    Red = 2
}

impl From<SemaphoreStates> for StepMachineLabel {
    fn from(state: SemaphoreStates) -> StepMachineLabel {
        StepMachineLabel::StepLabel(state as StepLabel)
    }
}