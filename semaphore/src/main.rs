extern crate step_machine;

use step_machine::{StepMachineLabel, Step, StepMachine};
use crate::steps::labels::SemaphoreStates;
use crate::steps::call::{green, yellow, red};

mod steps;

fn main() {
    let steps:Vec<(StepMachineLabel,Step<i32>)>  = vec![
        (SemaphoreStates::Green.into(), |x| green(x)),
        (SemaphoreStates::Yellow.into(), |x| yellow(x)),
        (SemaphoreStates::Red.into(), |x| red(x))
    ];

    let mut step_machine = StepMachine::<i32>::new(None, steps, None);
    let _ = step_machine.run(SemaphoreStates::Green.into());
}
