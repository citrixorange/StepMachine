extern crate step_machine;

use step_machine::{StepLabel, Step, StepMachine};
use crate::steps::labels::SemaphoreStates;
use crate::steps::call::{green, yellow, red};

mod steps;

fn main() {
    let steps:Vec<(StepLabel,Step<i32>)>  = vec![
        (SemaphoreStates::Green.into(), |x,y| green(x,y)),
        (SemaphoreStates::Yellow.into(), |x,y| yellow(x,y)),
        (SemaphoreStates::Red.into(), |x,y| red(x,y))
    ];

    let mut step_machine = StepMachine::<i32>::new(None, None, steps, None);
    let _ = step_machine.run(SemaphoreStates::Green.into());
}