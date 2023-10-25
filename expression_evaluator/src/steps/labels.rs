extern crate step_machine;

use step_machine::{StepLabel, StepMachineLabel};

#[repr(i32)]
pub enum ExpressionSteps {
    Add = 0,
    Subtract = 1,
    Multiply = 2,
    Divide = 3
}

impl From<ExpressionSteps> for StepMachineLabel {
    fn from(state: ExpressionSteps) -> StepMachineLabel {
        StepMachineLabel::StepLabel(state as StepLabel)
    }
}

impl From<ExpressionSteps> for StepLabel {
    fn from(state: ExpressionSteps) -> StepLabel {
        state as StepLabel
    }
}